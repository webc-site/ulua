//! Source: `tests/IrLowering.test.cpp`
//!
//! 从 Luau 源码到 IR 汇编文本的完整管线：
//! parse → compileOrThrow → bytecode → luau_load(VM) → getAssembly(X64_SystemV) → IR 文本，
//! 随后用 A64 target 再 lower 一遍（结果丢弃，仅验证不崩）。
//! hooks（vector/userdata 自定义 lowering）与 library 常量查找复刻自
//! `ConformanceIrHooks.h` 与 conformance 侧同名额子。

use alloc::string::{String, ToString as _};
use core::{
  ffi::{c_char, c_void},
  mem::{offset_of, size_of},
  ptr::{NonNull, null, null_mut},
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::{bytecode_builder::BytecodeBuilder, bytecode_encoder::NoopEncoder};
use ulua_code_gen::{
  enums::{
    host_metamethod::HostMetamethod, include_cfg_info::IncludeCfgInfo,
    include_ir_prefix::IncludeIrPrefix, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo, ir_cmd::IrCmd, target::Target,
  },
  functions::{
    get_assembly::{assembly_text, get_assembly},
    luau_codegen_create::luau_codegen_create,
    luau_codegen_supported::luau_codegen_supported,
    set_userdata_remapper::set_userdata_remapper,
  },
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    ir_builder::IrBuilder, ir_op::IrOp,
  },
};
use ulua_common::functions::c_str::cstr_bytes;
use ulua_compiler::{
  functions::{
    compile::compile,
    compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
    set_compile_constant::{
      set_compile_constant_boolean, set_compile_constant_nil, set_compile_constant_number,
      set_compile_constant_string, set_compile_constant_vector,
    },
  },
  records::compile_options::CompileOptions,
  type_aliases::compile_constant::CompileConstant,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
};

use crate::macros::cstr_names::{NAME_TEST, NAME_TEST_STR, NAME_VECTOR, NAME_VECTOR3};

/// userdata 类型名（NUL 结尾字节串，`*const c_char` 契约调用点 `.as_ptr().cast()`）。
const UD_VEC2: &[u8] = b"vec2\0";
const UD_COLOR: &[u8] = b"color\0";
const UD_MAT3: &[u8] = b"mat3\0";
const UD_VERTEX: &[u8] = b"vertex\0";
const UD_EXTRA: &[u8] = b"extra\0";

const LBC_TYPE_VECTOR: i32 = 8;
const LBC_TYPE_ANY: i32 = 15;
const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;

// LBC 类型码 u8 单源表（供各 bytecode-type 查询函数引用，替代原 fn 内局部 const）。
// ANY 双口径逐字保留：vector/metamethod 查询用 15，userdata bytecode-type 查询用 0。
const LBC8_NUMBER: u8 = 2;
const LBC8_VECTOR: u8 = 8;
const LBC8_ANY: u8 = 15;
const LBC8_ANY_UD: u8 = 0;
/// Vec2 userdata 的 LBC tagged 类型码（原 `userdata_index_to_type(Vec2::USERDATA_INDEX)`）。
const LBC8_UD_VEC2: u8 = LBC_TYPE_TAGGED_USERDATA_BASE + UserdataKind::VEC2;

// 各 get_codegen_assembly* 共用的 userdata / library 名表
// （顺序即 C++ 测试中的编译期 userdata 映射 vec2,color,mat3,vertex）。
const USERDATA_COMPILE_TYPES: [*const c_char; 5] = [
  UD_VEC2.as_ptr().cast(),
  UD_COLOR.as_ptr().cast(),
  UD_MAT3.as_ptr().cast(),
  UD_VERTEX.as_ptr().cast(),
  null(),
];
// 运行时 remapper 故意与编译期不同序（见 userdata_remapper）。
const USERDATA_RUN_TYPES: [*const c_char; 6] = [
  UD_EXTRA.as_ptr().cast(),
  UD_COLOR.as_ptr().cast(),
  UD_VEC2.as_ptr().cast(),
  UD_MAT3.as_ptr().cast(),
  UD_VERTEX.as_ptr().cast(),
  null(),
];
const LIBRARIES_WITH_CONSTANTS: [*const c_char; 4] = [
  NAME_VECTOR.as_ptr().cast(),
  NAME_VECTOR3.as_ptr().cast(),
  NAME_TEST.as_ptr().cast(),
  null(),
];

// ============================================================================
// library member 常量/类型查找（tests/IrLowering.test.cpp 顶部静态函数）
// ============================================================================

fn luau_library_type_lookup(library: &str, member: &str) -> i32 {
  if library == "Vector3" && (member == "xAxis" || member == "yAxis") {
    return LBC_TYPE_VECTOR;
  }

  LBC_TYPE_ANY
}

fn luau_library_constant_lookup(library: &str, member: &str, constant: *mut CompileConstant) {
  let const_ptr: CompileConstant = constant.cast();

  // (library, member) → 向量常量：命中即写入并返回，与原两段 `if + match member` 语义一致
  let vector_const = match (library, member) {
    ("vector", "zero") => Some([0.0, 0.0, 0.0, 0.0]),
    ("vector", "one") => Some([1.0, 1.0, 1.0, 0.0]),
    ("Vector3", "xAxis") => Some([1.0, 0.0, 0.0, 0.0]),
    ("Vector3", "yAxis") => Some([0.0, 1.0, 0.0, 0.0]),
    _ => None,
  };
  if let Some([x, y, z, w]) = vector_const {
    set_compile_constant_vector(const_ptr, x, y, z, w);
    return;
  }

  if library == "test" {
    match member {
      "some_nil" => set_compile_constant_nil(const_ptr),
      "some_boolean" => set_compile_constant_boolean(const_ptr, true),
      "some_number" => set_compile_constant_number(const_ptr, 4.75),
      "some_vector" => set_compile_constant_vector(const_ptr, 1.0, 2.0, 4.0, 8.0),
      "some_string" => {
        set_compile_constant_string(const_ptr, NAME_TEST_STR.as_ptr(), NAME_TEST_STR.len())
      }
      _ => {}
    }
  }
}

/// C 字符串 → `&str`：经 `cstr_bytes` 门面取 NUL 前原始字节再判 UTF-8 解码，
/// 读法本身不安全，存活期由调用点决定。
///
/// # Safety
/// `ptr` 为空，或指向在返回引用的整个存活期内保持有效的 NUL 结尾 C 字符串。
/// 回调传入的名字缓冲只在**该次回调调用期间**有效，故返回引用只允许在回调
/// 帧内使用；外传需 `String`/`Cow<'a, str>` 拷贝。
#[inline]
unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
  // Safety: 前置条件即 `ptr` 为空或指向合法 NUL 结尾串；`cstr_bytes` 空指针译空
  // 切片、非空返回 NUL 前借用（存活期 `'a`）。非法 UTF-8 降级空串（等价旧
  // `to_str().unwrap_or("")`）。
  from_utf8(unsafe { cstr_bytes(ptr) }).unwrap_or("")
}

/// # Safety
/// 回调契约：指针指向以 NUL 结尾的合法 C 字符串。
unsafe extern "C-unwind" fn luau_library_type_lookup_callback(
  library: *const u8,
  member: *const u8,
) -> i32 {
  // Safety: 两个缓冲在本回调帧内有效，返回值不出帧。
  let (library, member) = unsafe { (cstr_to_str(library.cast()), cstr_to_str(member.cast())) };
  luau_library_type_lookup(library, member)
}

/// # Safety
/// 回调契约：指针指向以 NUL 结尾的合法 C 字符串，constant 可写。
unsafe extern "C-unwind" fn luau_library_constant_lookup_callback(
  library: *const u8,
  member: *const u8,
  constant: *mut CompileConstant,
) {
  // Safety: 同上，引用仅在本回调帧内使用。
  let (library, member) = unsafe { (cstr_to_str(library.cast()), cstr_to_str(member.cast())) };
  luau_library_constant_lookup(library, member, constant);
}

/// # Safety
/// 回调契约：name 指向 name_length 字节的合法内存。
unsafe extern "C-unwind" fn userdata_remapper(
  _context: *mut c_void,
  name: *const c_char,
  name_length: usize,
) -> u8 {
  // Safety: luau 编译 C ABI 回调契约：remapper 的 name/name_length 恒指向编译器
  // 传入的待映射类型名缓冲（本帧内可读、界内），借用不出帧（[`member_to_str`]
  // 契约）。
  let name_str = unsafe { member_to_str(name, name_length) };
  // 运行时映射故意与编译期映射（vec2,color,mat3,vertex）不同序。
  match name_str {
    "extra" => 0,
    "color" => 1,
    "vec2" => 2,
    "mat3" => 3,
    "vertex" => 4,
    _ => 0xff,
  }
}

// ============================================================================
// ConformanceIrHooks：vector/userdata 自定义 lowering
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum UserdataKind {
  Extra = 0,
  Color = 1,
  Vec2 = 2,
  Mat3 = 3,
  Vertex = 4,
}

impl UserdataKind {
  const VEC2: u8 = Self::Vec2 as u8;

  #[inline]
  fn from_type(ty: u8) -> Option<Self> {
    // 下溢回绕后不会匹配任何 kUserdata* 常量
    match ty.wrapping_sub(LBC_TYPE_TAGGED_USERDATA_BASE) {
      0 => Some(Self::Extra),
      1 => Some(Self::Color),
      2 => Some(Self::Vec2),
      3 => Some(Self::Mat3),
      4 => Some(Self::Vertex),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
struct Vec2 {
  x: f32,
  y: f32,
}

impl Vec2 {
  const TAG: i32 = 12;
  const SIZE: usize = size_of::<Self>();
  const OFFSET_X: usize = offset_of!(Self, x);
  const OFFSET_Y: usize = offset_of!(Self, y);

  #[inline]
  fn check_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    let tag = build.const_int(Self::TAG);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckUserdataTag, udata, tag, exit);
  }

  #[inline]
  fn read_field(build: &mut IrBuilder, udata: IrOp, offset: usize) -> IrOp {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(LuaType::UserData as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BufferReadf32, udata, offset, tag)
  }

  #[inline]
  fn write_field(build: &mut IrBuilder, udata: IrOp, offset: usize, value: IrOp) {
    let offset = build.const_int(offset as i32);
    let tag = build.const_tag(LuaType::UserData as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BufferWritef32, udata, offset, value, tag);
  }

  #[inline]
  fn new_userdata(build: &mut IrBuilder) -> IrOp {
    // cpp ConformanceIrHooks.h 的 NEW_USERDATA 前无 CHECK_GC，期望串以 cpp 为准
    let size = build.const_int(Self::SIZE as i32);
    let tag = build.const_int(Self::TAG);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::NewUserdata, size, tag)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
struct Vertex {
  pos: [f32; 3],
  normal: [f32; 3],
  uv: [f32; 2],
}

impl Vertex {
  const TAG: i32 = 13;

  const OFFSET_POS: usize = offset_of!(Self, pos);
  const OFFSET_POS_X: usize = Self::OFFSET_POS;
  const OFFSET_POS_Y: usize = Self::OFFSET_POS + size_of::<f32>();
  const OFFSET_POS_Z: usize = Self::OFFSET_POS + 2 * size_of::<f32>();

  const OFFSET_NORMAL: usize = offset_of!(Self, normal);
  const OFFSET_NORMAL_X: usize = Self::OFFSET_NORMAL;
  const OFFSET_NORMAL_Y: usize = Self::OFFSET_NORMAL + size_of::<f32>();
  const OFFSET_NORMAL_Z: usize = Self::OFFSET_NORMAL + 2 * size_of::<f32>();

  const OFFSET_UV: usize = offset_of!(Self, uv);
  const OFFSET_UV_X: usize = Self::OFFSET_UV;
  const OFFSET_UV_Y: usize = Self::OFFSET_UV + size_of::<f32>();

  #[inline]
  fn check_tag(build: &mut IrBuilder, udata: IrOp, pcpos: i32) {
    let tag = build.const_int(Self::TAG);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckUserdataTag, udata, tag, exit);
  }
}

/// `match member { … => ty, _ => fallback }` 同构查表的单源等价物：线性查表命中即返，未命中回 `fallback`。
#[inline]
fn member_type_lookup(member: &str, table: &[(&str, u8)], fallback: u8) -> u8 {
  table
    .iter()
    .find(|(name, _)| *name == member)
    .map_or(fallback, |(_, ty)| *ty)
}

// member → bytecode-type 定表（未命中回各函数自身 ANY 口径）
const VEC_ACCESS_MEMBERS: &[(&str, u8)] = &[("Magnitude", LBC8_NUMBER), ("Unit", LBC8_VECTOR)];
const VEC_NAMECALL_MEMBERS: &[(&str, u8)] = &[("Dot", LBC8_NUMBER), ("Cross", LBC8_VECTOR)];

fn vector_access_bytecode_type(member: &str) -> u8 {
  member_type_lookup(member, VEC_ACCESS_MEMBERS, LBC8_ANY)
}

fn vector_namecall_bytecode_type(member: &str) -> u8 {
  member_type_lookup(member, VEC_NAMECALL_MEMBERS, LBC8_ANY)
}

/// 读 vm 向量寄存器的 f32 分量 (x, y, z) 与平方和 x²+y²+z²（Magnitude/Unit 共用）。
fn load_vec3(build: &mut IrBuilder, src: IrOp) -> (IrOp, IrOp, IrOp, IrOp) {
  let (c0, c4, c8) = (build.const_int(0), build.const_int(4), build.const_int(8));
  let x = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src, c0);
  let y = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src, c4);
  let z = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src, c8);

  let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
  let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
  let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z, z);
  let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, sum_xy, z2);
  (x, y, z, sum)
}

fn vector_access(
  build: &mut IrBuilder,
  member: &str,
  result_reg: i32,
  source_reg: i32,
  _pcpos: i32,
) -> bool {
  match member {
    "Magnitude" => {
      let src = build.vm_reg(source_reg as u8);
      let (_, _, _, sum) = load_vec3(build, src);

      let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
      let mag_num = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, mag);

      store_number(build, result_reg, mag_num);
      true
    }
    "Unit" => {
      let src = build.vm_reg(source_reg as u8);
      let (x, y, z, sum) = load_vec3(build, src);

      let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
      let one = build.const_double(1.0);
      let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);

      let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, inv);
      let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, inv);
      let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z, inv);

      store_vector(build, result_reg, xr, yr, zr);
      true
    }
    _ => false,
  }
}

fn vector_namecall(
  build: &mut IrBuilder,
  member: &str,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  if params != 2 || results > 1 {
    return false;
  }

  match member {
    "Dot" => {
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(arg_reg, LuaType::Vector as u8, exit);

      let src_reg = build.vm_reg(source_reg as u8);
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);

      let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c0);
      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c0);
      let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, x2);

      let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c4);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c4);
      let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, y2);

      let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c8);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c8);
      let zz = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, z2);

      let sum_xy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, xx, yy);
      let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, sum_xy, zz);

      let num = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, sum);
      store_number(build, arg_res_reg, num);
      adjust_multret(build, arg_res_reg, results);

      true
    }
    "Cross" => {
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(arg_reg, LuaType::Vector as u8, exit);

      let src_reg = build.vm_reg(source_reg as u8);
      let arg_reg = build.vm_reg((arg_res_reg + 2) as u8);
      let c0 = build.const_int(0);
      let c4 = build.const_int(4);
      let c8 = build.const_int(8);

      let x1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c0);
      let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c0);
      let y1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c4);
      let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c4);
      let z1 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, src_reg, c8);
      let z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, arg_reg, c8);

      let y1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, z2);
      let z1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, y2);
      let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, y1z2, z1y2);

      let z1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, z1, x2);
      let x1z2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, z2);
      let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, z1x2, x1z2);

      let x1y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x1, y2);
      let y1x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y1, x2);
      let zr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubFloat, x1y2, y1x2);

      store_vector(build, arg_res_reg, xr, yr, zr);
      adjust_multret(build, arg_res_reg, results);

      true
    }
    _ => false,
  }
}

fn store_number(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, result, value);
  let tag = build.const_tag(LuaType::Number as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

fn store_userdata(build: &mut IrBuilder, result_reg: i32, value: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, result, value);
  let tag = build.const_tag(LuaType::UserData as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

fn store_vector(build: &mut IrBuilder, result_reg: i32, x: IrOp, y: IrOp, z: IrOp) {
  let result = build.vm_reg(result_reg as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, result, x, y, z);
  let tag = build.const_tag(LuaType::Vector as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTag, result, tag);
}

/// 读 Vec2 userdata 分量 (x, y) 与平方和 x²+y²（Magnitude/Unit 共用）。
fn vec2_xy_sum2(build: &mut IrBuilder, udata: IrOp) -> (IrOp, IrOp, IrOp) {
  let x = Vec2::read_field(build, udata, Vec2::OFFSET_X);
  let y = Vec2::read_field(build, udata, Vec2::OFFSET_Y);
  let x2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, x);
  let y2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, y);
  let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddFloat, x2, y2);
  (x, y, sum)
}

fn userdata_access(
  build: &mut IrBuilder,
  ty: u8,
  member: &str,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  match UserdataKind::from_type(ty) {
    Some(UserdataKind::Vec2) => match member {
      "X" | "Y" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let field_offset = if member == "X" {
          Vec2::OFFSET_X
        } else {
          Vec2::OFFSET_Y
        };
        let value = Vec2::read_field(build, udata, field_offset);
        let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
        store_number(build, result_reg, value);
        true
      }
      "Magnitude" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let (_, _, sum) = vec2_xy_sum2(build, udata);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, mag);
        store_number(build, result_reg, mag);
        true
      }
      "Unit" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata, pcpos);

        let (x, y, sum) = vec2_xy_sum2(build, udata);
        let mag = build.inst_ir_cmd_ir_op(IrCmd::SqrtFloat, sum);
        let one = build.const_double(1.0);
        let inv = build.inst_ir_cmd_ir_op_ir_op(IrCmd::DivFloat, one, mag);
        let xr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, x, inv);
        let yr = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulFloat, y, inv);

        let result = Vec2::new_userdata(build);
        Vec2::write_field(build, result, Vec2::OFFSET_X, xr);
        Vec2::write_field(build, result, Vec2::OFFSET_Y, yr);
        store_userdata(build, result_reg, result);
        true
      }
      _ => false,
    },
    Some(UserdataKind::Vertex) => match member {
      "pos" | "normal" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vertex::check_tag(build, udata, pcpos);

        let (x_off, y_off, z_off) = if member == "pos" {
          (
            Vertex::OFFSET_POS_X,
            Vertex::OFFSET_POS_Y,
            Vertex::OFFSET_POS_Z,
          )
        } else {
          (
            Vertex::OFFSET_NORMAL_X,
            Vertex::OFFSET_NORMAL_Y,
            Vertex::OFFSET_NORMAL_Z,
          )
        };

        let x = Vec2::read_field(build, udata, x_off);
        let y = Vec2::read_field(build, udata, y_off);
        let z = Vec2::read_field(build, udata, z_off);
        store_vector(build, result_reg, x, y, z);
        true
      }
      "uv" => {
        let source = build.vm_reg(source_reg as u8);
        let udata = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vertex::check_tag(build, udata, pcpos);

        let x = Vec2::read_field(build, udata, Vertex::OFFSET_UV_X);
        let y = Vec2::read_field(build, udata, Vertex::OFFSET_UV_Y);

        let result = Vec2::new_userdata(build);
        Vec2::write_field(build, result, Vec2::OFFSET_X, x);
        Vec2::write_field(build, result, Vec2::OFFSET_Y, y);
        store_userdata(build, result_reg, result);
        true
      }
      _ => false,
    },
    _ => false,
  }
}

// userdata kind → member 定表（未命中回 LBC8_ANY_UD）
const UD_COLOR_MEMBERS: &[(&str, u8)] =
  &[("R", LBC8_NUMBER), ("G", LBC8_NUMBER), ("B", LBC8_NUMBER)];
const UD_VEC2_MEMBERS: &[(&str, u8)] = &[
  ("X", LBC8_NUMBER),
  ("Y", LBC8_NUMBER),
  ("Magnitude", LBC8_NUMBER),
  ("Unit", LBC8_UD_VEC2),
];
const UD_MAT3_MEMBERS: &[(&str, u8)] = &[
  ("Row1", LBC8_VECTOR),
  ("Row2", LBC8_VECTOR),
  ("Row3", LBC8_VECTOR),
];
const UD_VERTEX_MEMBERS: &[(&str, u8)] = &[
  ("pos", LBC8_VECTOR),
  ("normal", LBC8_VECTOR),
  ("uv", LBC8_UD_VEC2),
];
const UD_VEC2_NAMECALL_MEMBERS: &[(&str, u8)] = &[("Dot", LBC8_NUMBER), ("Min", LBC8_UD_VEC2)];

fn userdata_access_bytecode_type(ty: u8, member: &str) -> u8 {
  let table = match UserdataKind::from_type(ty) {
    Some(UserdataKind::Color) => UD_COLOR_MEMBERS,
    Some(UserdataKind::Vec2) => UD_VEC2_MEMBERS,
    Some(UserdataKind::Mat3) => UD_MAT3_MEMBERS,
    Some(UserdataKind::Vertex) => UD_VERTEX_MEMBERS,
    _ => return LBC8_ANY_UD,
  };
  member_type_lookup(member, table, LBC8_ANY_UD)
}

fn userdata_metamethod_bytecode_type(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8 {
  match method {
    HostMetamethod::Add | HostMetamethod::Sub | HostMetamethod::Mul | HostMetamethod::Div => {
      if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2)
        || UserdataKind::from_type(rhs_ty) == Some(UserdataKind::Vec2)
      {
        LBC8_UD_VEC2
      } else {
        LBC8_ANY
      }
    }
    HostMetamethod::Minus if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2) => {
      LBC8_UD_VEC2
    }
    _ => LBC8_ANY,
  }
}

fn userdata_namecall_bytecode_type(ty: u8, member: &str) -> u8 {
  match UserdataKind::from_type(ty) {
    Some(UserdataKind::Vec2) => member_type_lookup(member, UD_VEC2_NAMECALL_MEMBERS, LBC8_ANY_UD),
    _ => LBC8_ANY_UD,
  }
}

fn userdata_metamethod(
  build: &mut IrBuilder,
  lhs_ty: u8,
  rhs_ty: u8,
  result_reg: i32,
  lhs: IrOp,
  rhs: IrOp,
  method: HostMetamethod,
  pcpos: i32,
) -> bool {
  match method {
    HostMetamethod::Add | HostMetamethod::Mul => {
      if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2)
        && UserdataKind::from_type(rhs_ty) == Some(UserdataKind::Vec2)
      {
        let vm_exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(lhs, LuaType::UserData as u8, vm_exit);
        let vm_exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(rhs, LuaType::UserData as u8, vm_exit);

        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, lhs);
        Vec2::check_tag(build, udata1, pcpos);

        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, rhs);
        Vec2::check_tag(build, udata2, pcpos);

        let x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);

        let cmd = if method == HostMetamethod::Add {
          IrCmd::AddFloat
        } else {
          IrCmd::MulFloat
        };
        let mx = build.inst_ir_cmd_ir_op_ir_op(cmd, x1, x2);

        let y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        let my = build.inst_ir_cmd_ir_op_ir_op(cmd, y1, y2);

        let udatar = Vec2::new_userdata(build);
        Vec2::write_field(build, udatar, Vec2::OFFSET_X, mx);
        Vec2::write_field(build, udatar, Vec2::OFFSET_Y, my);

        store_userdata(build, result_reg, udatar);

        true
      } else {
        false
      }
    }
    HostMetamethod::Minus if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2) => {
      let vm_exit = build.vm_exit(pcpos as u32);
      build.load_and_check_tag(lhs, LuaType::UserData as u8, vm_exit);

      let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, lhs);
      Vec2::check_tag(build, udata1, pcpos);

      let x = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
      let y = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);

      let mx = build.inst_ir_cmd_ir_op(IrCmd::UnmFloat, x);
      let my = build.inst_ir_cmd_ir_op(IrCmd::UnmFloat, y);

      let udatar = Vec2::new_userdata(build);
      Vec2::write_field(build, udatar, Vec2::OFFSET_X, mx);
      Vec2::write_field(build, udatar, Vec2::OFFSET_Y, my);

      store_userdata(build, result_reg, udatar);

      true
    }
    _ => false,
  }
}

fn adjust_multret(build: &mut IrBuilder, arg_res_reg: i32, results: i32) {
  if results == LUA_MULTRET {
    let reg = build.vm_reg(arg_res_reg as u8);
    let count = build.const_int(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg, count);
  }
}

fn userdata_namecall(
  build: &mut IrBuilder,
  ty: u8,
  member: &str,
  arg_res_reg: i32,
  source_reg: i32,
  _params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  match UserdataKind::from_type(ty) {
    Some(UserdataKind::Vec2) => match member {
      "Dot" => {
        let source = build.vm_reg(source_reg as u8);
        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata1, pcpos);

        let arg = build.vm_reg((source_reg + 1) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg, LuaType::UserData as u8, exit);

        let arg = build.vm_reg((source_reg + 1) as u8);
        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, arg);
        Vec2::check_tag(build, udata2, pcpos);

        let mut x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let mut x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);
        x1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x1);
        x2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x2);
        let xx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, x1, x2);

        let mut y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let mut y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        y1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y1);
        y2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y2);
        let yy = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MulNum, y1, y2);
        let sum = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, xx, yy);

        store_number(build, arg_res_reg, sum);
        adjust_multret(build, arg_res_reg, results);
        true
      }
      "Min" => {
        let source = build.vm_reg(source_reg as u8);
        let udata1 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, source);
        Vec2::check_tag(build, udata1, pcpos);

        let arg = build.vm_reg((source_reg + 1) as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(arg, LuaType::UserData as u8, exit);

        let arg = build.vm_reg((source_reg + 1) as u8);
        let udata2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, arg);
        Vec2::check_tag(build, udata2, pcpos);

        let mut x1 = Vec2::read_field(build, udata1, Vec2::OFFSET_X);
        let mut x2 = Vec2::read_field(build, udata2, Vec2::OFFSET_X);
        x1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x1);
        x2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, x2);
        let mx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, x1, x2);

        let mut y1 = Vec2::read_field(build, udata1, Vec2::OFFSET_Y);
        let mut y2 = Vec2::read_field(build, udata2, Vec2::OFFSET_Y);
        y1 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y1);
        y2 = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, y2);
        let my = build.inst_ir_cmd_ir_op_ir_op(IrCmd::MinNum, y1, y2);

        let mx = build.inst_ir_cmd_ir_op(IrCmd::NumToFloat, mx);
        let my = build.inst_ir_cmd_ir_op(IrCmd::NumToFloat, my);

        let udata_result = Vec2::new_userdata(build);
        Vec2::write_field(build, udata_result, Vec2::OFFSET_X, mx);
        Vec2::write_field(build, udata_result, Vec2::OFFSET_Y, my);

        store_userdata(build, arg_res_reg, udata_result);
        adjust_multret(build, arg_res_reg, results);
        true
      }
      _ => false,
    },
    _ => false,
  }
}

// hooks 的 extern 回调包装

/// codegen 回调的 `builder` 裸指针 → `&mut IrBuilder` 重借：本文件唯一的
/// builder 解引用收口（cpp 各 `Host*Callback` 的 `(IrBuilder*)builder` 同形）。
///
/// # Safety
/// `builder` 为 codegen C ABI 回调契约交付的当前编译帧非空存活 `IrBuilder*`，
/// 且返回的独占借用仅在该回调帧内存活（借用不逃逸、单线程无并发别名）。
#[inline]
unsafe fn builder_mut<'a>(builder: *mut IrBuilder) -> &'a mut IrBuilder {
  // Safety: 转调即函数级 `# Safety` 契约本身。
  unsafe { &mut *builder }
}

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn vector_access_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  vector_access_bytecode_type(m)
}

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn vector_namecall_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  vector_namecall_bytecode_type(m)
}

/// # Safety
/// 回调契约：builder/member 指针合法。
unsafe extern "C-unwind" fn vector_access_callback(
  builder: *mut IrBuilder,
  member: *const c_char,
  member_length: usize,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  // Safety: builder 为 codegen C ABI 回调帧内非空存活 IrBuilder*（契约见
  // [`builder_mut`]），&mut 重借用仅在帧内发射向量访问指令。
  unsafe { vector_access(builder_mut(builder), m, result_reg, source_reg, pcpos) }
}

/// # Safety
/// 回调契约：builder/member 指针合法。
unsafe extern "C-unwind" fn vector_namecall_callback(
  builder: *mut IrBuilder,
  member: *const c_char,
  member_length: usize,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  // Safety: 同 [`builder_mut`] 契约——builder 为回调帧内非空存活 IrBuilder*，
  // 独占重借用仅在帧内发射 namecall 指令；member 已在上方物化为局部值。
  unsafe {
    vector_namecall(
      builder_mut(builder),
      m,
      arg_res_reg,
      source_reg,
      params,
      results,
      pcpos,
    )
  }
}

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn userdata_access_bytecode_type_callback(
  ty: u8,
  member: *const c_char,
  member_length: usize,
) -> u8 {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  userdata_access_bytecode_type(ty, m)
}

/// # Safety
/// 回调契约：参数满足 C++ HostUserdataMetamethodBytecodeType 契约。
unsafe extern "C-unwind" fn userdata_metamethod_bytecode_type_callback(
  lhs_ty: u8,
  rhs_ty: u8,
  method: HostMetamethod,
) -> u8 {
  userdata_metamethod_bytecode_type(lhs_ty, rhs_ty, method)
}

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn userdata_namecall_bytecode_type_callback(
  ty: u8,
  member: *const c_char,
  member_length: usize,
) -> u8 {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  userdata_namecall_bytecode_type(ty, m)
}

/// # Safety
/// 回调契约：builder/member 指针合法。
unsafe extern "C-unwind" fn userdata_access_callback(
  builder: *mut IrBuilder,
  ty: u8,
  member: *const c_char,
  member_length: usize,
  result_reg: i32,
  source_reg: i32,
  pcpos: i32,
) -> bool {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  // Safety: 契约见 [`builder_mut`]——builder 为回调帧内非空存活 IrBuilder*，
  // 独占重借用仅在帧内发射 userdata access 指令。
  unsafe { userdata_access(builder_mut(builder), ty, m, result_reg, source_reg, pcpos) }
}

/// # Safety
/// 回调契约：builder/member 指针合法。
unsafe extern "C-unwind" fn userdata_metamethod_callback(
  builder: *mut IrBuilder,
  lhs_ty: u8,
  rhs_ty: u8,
  result_reg: i32,
  lhs: IrOp,
  rhs: IrOp,
  method: HostMetamethod,
  pcpos: i32,
) -> bool {
  // Safety: 契约见 [`builder_mut`]——builder 为当前回调帧非空存活 IrBuilder*，
  // 独占重借用仅在帧内发射 metamethod 调用；lhs/rhs 为编译器产出的合法 IrOp 值。
  let builder = unsafe { builder_mut(builder) };
  userdata_metamethod(builder, lhs_ty, rhs_ty, result_reg, lhs, rhs, method, pcpos)
}

/// # Safety
/// 回调契约：builder/member 指针合法。
unsafe extern "C-unwind" fn userdata_namecall_callback(
  builder: *mut IrBuilder,
  ty: u8,
  member: *const c_char,
  member_length: usize,
  arg_res_reg: i32,
  source_reg: i32,
  params: i32,
  results: i32,
  pcpos: i32,
) -> bool {
  // Safety: 成员名缓冲在本回调帧内有效，引用不出帧。
  let m = unsafe { member_to_str(member, member_length) };
  // Safety: 契约见 [`builder_mut`]——builder 为当前回调帧非空存活 IrBuilder*，
  // 独占重借用仅在帧内发射 userdata namecall；成员名已在上方物化。
  let builder = unsafe { builder_mut(builder) };
  userdata_namecall(
    builder,
    ty,
    m,
    arg_res_reg,
    source_reg,
    params,
    results,
    pcpos,
  )
}

/// 长度+指针的 C 名字缓冲 → `&str`：读法本身不安全，存活期由调用点决定。
///
/// # Safety
/// `ptr` 为空，或指向 `len` 字节可读内存（成员名缓冲由编译器在本回调调用期间持有）。
/// 返回引用只允许在回调帧内使用；外传需 `String`/`Cow<'a, str>` 拷贝。
#[inline]
unsafe fn member_to_str<'a>(ptr: *const c_char, len: usize) -> &'a str {
  if ptr.is_null() || len == 0 {
    ""
  } else {
    // Safety: 前置条件保证 `[ptr, ptr + len)` 可读。
    let bytes = unsafe { from_raw_parts(ptr.cast::<u8>(), len) };
    from_utf8(bytes).unwrap_or("")
  }
}

// ============================================================================
// LoweringFixture
// ============================================================================

#[derive(Debug, Clone)]
pub struct LoweringFixture {
  pub compilation_options: CompileOptions,
  pub compilation_options_c: CompileOptions,
  pub assembly_options: AssemblyOptions,
}

/// RAII 守卫：退出时关闭 LuaState（对应 C++ unique_ptr + lua_close）。
/// 字段用 `NonNull` 承载：[`StateGuard::new`] 判空后才构造，空 VM 直接走
/// `None` 分支，裸指针可空哨兵由类型消掉。
struct StateGuard(NonNull<LuaState>);

impl StateGuard {
  fn new() -> Option<Self> {
    NonNull::new(lua_l_newstate()).map(Self)
  }

  fn as_ptr(&self) -> *mut LuaState {
    self.0.as_ptr()
  }

  /// 在守卫保有的存活 VM 上加载字节码：本文件 `luau_load` C ABI 契约的唯一
  /// 物化点（`"name"` 为静态源名；`bytecode` 借用即存活证明，覆盖本次调用；
  /// VM 存活与非空由 `StateGuard` 不变式给出）。返回码交调用点按原顺序断言。
  fn load(&self, bytecode: &[u8]) -> i32 {
    // Safety: 见方法文档——self.0 非空存活至守卫 drop，切片界内由入参借用
    // 证明，单线程帧内使用。
    unsafe { luau_load(self.as_ptr(), "name", bytecode, 0) }
  }

  /// 取该 VM 最近加载 chunk 的 IR 文本：本文件 `get_assembly` C ABI 契约的
  /// 唯一物化点（函数槽 `-1`、null context 按 cpp 约定不解引用；options 为
  /// 调用方交来的本地值拷贝，结果立即拷出文本，VM 在调用点后仍由守卫持有）。
  fn assembly(&self, options: AssemblyOptions) -> String {
    assembly_text(
      // Safety: 见方法文档——VM 存活由守卫保证，null context 按契约不解引用。
      unsafe { get_assembly(self.as_ptr(), -1, options, null_mut()) },
    )
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    // Safety: self.0 经 NonNull::new 判空后构造，guard 不可 Copy、唯一消费者；
    // Drop 是 VM 唯一回收点，至多执行一次，lua_close 关闭该存活 VM 且此后无人
    // 再引用。
    unsafe { lua_close(self.0.as_ptr()) };
  }
}

fn configure_compile_options(
  options: &mut CompileOptions,
  userdata_types: *const *const c_char,
  libraries: *const *const c_char,
  debug_level: i32,
  optimization_level: i32,
) {
  options.optimization_level = optimization_level;
  options.debug_level = debug_level;
  options.vector_ctor = NAME_VECTOR.as_ptr().cast();
  options.vector_type = NAME_VECTOR.as_ptr().cast();
  options.userdata_types = userdata_types;
  options.libraries_with_known_members = libraries;
  options.library_member_type_cb = Some(luau_library_type_lookup_callback);
  options.library_member_constant_cb = Some(luau_library_constant_lookup_callback);
}

fn configure_codegen_options(userdata_types: *const *const c_char) -> CompilationOptions {
  let mut options = CompilationOptions::default();
  options.hooks.vector_access_bytecode_type = Some(vector_access_bytecode_type_callback);
  options.hooks.vector_namecall_bytecode_type = Some(vector_namecall_bytecode_type_callback);
  options.hooks.vector_access = Some(vector_access_callback);
  options.hooks.vector_namecall = Some(vector_namecall_callback);
  options.hooks.userdata_access_bytecode_type = Some(userdata_access_bytecode_type_callback);
  options.hooks.userdata_metamethod_bytecode_type =
    Some(userdata_metamethod_bytecode_type_callback);
  options.hooks.userdata_namecall_bytecode_type = Some(userdata_namecall_bytecode_type_callback);
  options.hooks.userdata_access = Some(userdata_access_callback);
  options.hooks.userdata_metamethod = Some(userdata_metamethod_callback);
  options.hooks.userdata_namecall = Some(userdata_namecall_callback);
  options.userdata_types = userdata_types;
  options
}

fn make_assembly_options(
  compilation_options: CompilationOptions,
  include_ir_types: bool,
) -> AssemblyOptions {
  AssemblyOptions {
    target: Target::X64SystemV,
    compilation_options,
    output_binary: false,
    include_assembly: false,
    include_ir: true,
    include_outlined_code: false,
    include_ir_types,
    include_ir_prefix: IncludeIrPrefix::No,
    include_use_info: IncludeUseInfo::No,
    include_cfg_info: IncludeCfgInfo::No,
    include_reg_flow_info: IncludeRegFlowInfo::No,
    annotator: None,
    annotator_context: null_mut(),
  }
}

fn clip_assembly_to_first_return(mut assembly: String) -> String {
  if let Some(pos) = assembly.find("RETURN")
    && let Some(newline) = assembly[pos..].find('\n')
  {
    assembly.truncate(pos + newline + 1);
  }
  assembly
}

impl Default for LoweringFixture {
  fn default() -> Self {
    Self {
      compilation_options: default_compile_options(),
      compilation_options_c: default_compile_options(),
      assembly_options: make_assembly_options(CompilationOptions::default(), false),
    }
  }
}

/// C++ LoweringFixture 成员初始化列表里的 CompileOptions 初值。
fn default_compile_options() -> CompileOptions {
  CompileOptions {
    optimization_level: 2,
    debug_level: 1,
    type_info_level: 1,
    ..Default::default()
  }
}

impl LoweringFixture {
  /// 新建保活 VM、按需初始化 codegen remapper 并加载字节码：两个
  /// `get_codegen_assembly*` 的共同前段（原逐字重复的 guard→init→load 样板
  /// 收成一处）。`luau_load` 返回码原样交回，断言/释放顺序留在调用点，与
  /// 原行为逐字一致。
  fn load_state(&self, bytecode: &[u8]) -> (StateGuard, i32) {
    let state = StateGuard::new().expect("lua state allocation failed");
    if luau_codegen_supported() != 0 {
      // 类型 remapper 依赖 codegen 运行时
      // Safety: state.0 为刚新建且存活的 LuaState（StateGuard 不变式，
      // 对应 cpp C API 契约），两调用仅在守卫存续的帧内使用。
      unsafe {
        luau_codegen_create(state.as_ptr());
        set_userdata_remapper(state.as_ptr(), null_mut(), Some(userdata_remapper));
      }
    }
    let load_result = state.load(bytecode);
    (state, load_result)
  }

  /// 源码 → bytecode → VM 加载 → X64_SystemV IR 文本（附带 A64 复跑）。
  pub fn get_codegen_assembly(
    &mut self,
    source: &str,
    include_ir_types: bool,
    debug_level: i32,
    optimization_level: i32,
    clip_to_first_return: bool,
  ) -> String {
    configure_compile_options(
      &mut self.compilation_options,
      USERDATA_COMPILE_TYPES.as_ptr(),
      LIBRARIES_WITH_CONSTANTS.as_ptr(),
      debug_level,
      optimization_level,
    );

    let mut bcb = BytecodeBuilder::new(None);
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut bcb,
      source,
      &self.compilation_options,
      &ParseOptions::default(),
    );

    let bytecode = bcb.get_bytecode();
    let (state, load_result) = self.load_state(bytecode);
    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(USERDATA_RUN_TYPES.as_ptr());
    let mut assembly_options = make_assembly_options(codegen_options, include_ir_types);
    assembly_options.compilation_options.flags = self.assembly_options.compilation_options.flags;
    assembly_options.include_outlined_code = self.assembly_options.include_outlined_code;
    assembly_options.include_reg_flow_info = self.assembly_options.include_reg_flow_info;

    // IR 断言用稳定 target
    let result = state.assembly(assembly_options.clone());

    if luau_codegen_supported() != 0 {
      // 同时验证另一 target 也能正确 lower
      assembly_options.target = Target::A64;
      state.assembly(assembly_options);
    }

    if clip_to_first_return {
      clip_assembly_to_first_return(result)
    } else {
      result
    }
  }

  /// 经 safe 入口 `compile`（原 C ABI `luau_compile` 镜像的等价 Rust 形态）编译源码后取 IR 文本。
  pub fn get_codegen_assembly_using_c_api(
    &mut self,
    source: &str,
    include_ir_types: bool,
    debug_level: i32,
  ) -> String {
    self.compilation_options_c.optimization_level = 2;
    self.compilation_options_c.debug_level = debug_level;
    self.compilation_options_c.type_info_level = 1;

    let compile_options = CompileOptions {
      optimization_level: 2,
      debug_level,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: NAME_VECTOR.as_ptr().cast(),
      vector_type: NAME_VECTOR.as_ptr().cast(),
      mutable_globals: null(),
      userdata_types: USERDATA_COMPILE_TYPES.as_ptr(),
      libraries_with_known_members: LIBRARIES_WITH_CONSTANTS.as_ptr(),
      library_member_type_cb: Some(luau_library_type_lookup_callback),
      library_member_constant_cb: Some(luau_library_constant_lookup_callback),
      disabled_builtins: null(),
    };

    let bytecode = compile(
      source.as_bytes(),
      &compile_options,
      &ParseOptions::default(),
      NoopEncoder,
    );

    let (state, load_result) = self.load_state(&bytecode);
    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(USERDATA_RUN_TYPES.as_ptr());
    let assembly_options = make_assembly_options(codegen_options, include_ir_types);

    state.assembly(assembly_options)
  }

  /// 取最后一个函数的头部（IR 类型注解），debugLevel 固定 2。
  pub fn get_codegen_header(&mut self, source: &str) -> String {
    let mut assembly = self.get_codegen_assembly(source, true, 2, 2, false);

    // 跳到最后一个函数：原「find 命中即重切」循环坍为一次 `rfind`，产出与
    // 逐次切割的最终值相同（跳过位置 0 的表头 token，与原 `[1..]` 起点一致）
    if let Some(pos) = assembly[1..].rfind("; function ") {
      assembly = assembly[pos + 1..].to_string();
    }

    let bytecode_start = assembly
      .find("bb_bytecode_0:")
      .or_else(|| assembly.find("bb_0:"))
      .unwrap_or_else(|| panic!("bb_bytecode_0: not found"));

    assembly[..bytecode_start].to_string()
  }
}
