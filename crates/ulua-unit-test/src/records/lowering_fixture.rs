//! Source: `tests/IrLowering.test.cpp`
//!
//! 从 Luau 源码到 IR 汇编文本的完整管线：
//! parse → compileOrThrow → bytecode → luau_load(VM) → getAssembly(X64_SystemV) → IR 文本，
//! 随后用 A64 target 再 lower 一遍（结果丢弃，仅验证不崩）。
//! hooks（vector/userdata 自定义 lowering）与 library 常量查找复刻自
//! `ConformanceIrHooks.h` 与 conformance 侧同名额子。

use alloc::string::{String, ToString as _};
use core::{
  ffi::{CStr, c_char, c_void},
  mem::{offset_of, size_of},
  ptr::{null, null_mut},
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
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
use ulua_compiler::{
  functions::{
    compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
    luau_compile::luau_compile, set_compile_constant_boolean::set_compile_constant_boolean,
    set_compile_constant_nil::set_compile_constant_nil,
    set_compile_constant_number::set_compile_constant_number,
    set_compile_constant_string::set_compile_constant_string,
    set_compile_constant_vector::set_compile_constant_vector,
  },
  records::{compile_options::CompileOptions, lua_compile_options::LuaCompileOptions},
  type_aliases::compile_constant::CompileConstant,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::lua_State,
};

const LBC_TYPE_VECTOR: i32 = 8;
const LBC_TYPE_ANY: i32 = 15;
const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;

// 各 get_codegen_assembly* 共用的 userdata / library 名表
// （顺序即 C++ 测试中的编译期 userdata 映射 vec2,color,mat3,vertex）。
const USERDATA_COMPILE_TYPES: [*const c_char; 5] = [
  c"vec2".as_ptr(),
  c"color".as_ptr(),
  c"mat3".as_ptr(),
  c"vertex".as_ptr(),
  null(),
];
// 运行时 remapper 故意与编译期不同序（见 userdata_remapper）。
const USERDATA_RUN_TYPES: [*const c_char; 6] = [
  c"extra".as_ptr(),
  c"color".as_ptr(),
  c"vec2".as_ptr(),
  c"mat3".as_ptr(),
  c"vertex".as_ptr(),
  null(),
];
const LIBRARIES_WITH_CONSTANTS: [*const c_char; 4] = [
  c"vector".as_ptr(),
  c"Vector3".as_ptr(),
  c"test".as_ptr(),
  null(),
];
const VECTOR_C: *const c_char = c"vector".as_ptr();

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
  let const_ptr = constant.cast::<c_void>();

  if library == "vector" {
    match member {
      "zero" => {
        set_compile_constant_vector(const_ptr, 0.0, 0.0, 0.0, 0.0);
        return;
      }
      "one" => {
        set_compile_constant_vector(const_ptr, 1.0, 1.0, 1.0, 0.0);
        return;
      }
      _ => {}
    }
  }

  if library == "Vector3" {
    match member {
      "xAxis" => {
        set_compile_constant_vector(const_ptr, 1.0, 0.0, 0.0, 0.0);
        return;
      }
      "yAxis" => {
        set_compile_constant_vector(const_ptr, 0.0, 1.0, 0.0, 0.0);
        return;
      }
      _ => {}
    }
  }

  if library == "test" {
    match member {
      "some_nil" => set_compile_constant_nil(const_ptr),
      "some_boolean" => set_compile_constant_boolean(const_ptr, true),
      "some_number" => set_compile_constant_number(const_ptr, 4.75),
      "some_vector" => set_compile_constant_vector(const_ptr, 1.0, 2.0, 4.0, 8.0),
      "some_string" => {
        let s = c"test".as_ptr();
        set_compile_constant_string(const_ptr, s, 4);
      }
      _ => {}
    }
  }
}

#[inline]
fn cstr_to_str(ptr: *const c_char) -> &'static str {
  if ptr.is_null() {
    ""
  } else {
    // SAFETY: 回调契约保证指针指向以 NUL 结尾的合法 C 字符串。
    unsafe { CStr::from_ptr(ptr) }.to_str().unwrap_or("")
  }
}

/// # Safety
/// 回调契约：指针指向以 NUL 结尾的合法 C 字符串。
unsafe extern "C-unwind" fn luau_library_type_lookup_callback(
  library: *const c_char,
  member: *const c_char,
) -> i32 {
  luau_library_type_lookup(cstr_to_str(library), cstr_to_str(member))
}

/// # Safety
/// 回调契约：指针指向以 NUL 结尾的合法 C 字符串，constant 可写。
unsafe extern "C-unwind" fn luau_library_constant_lookup_callback(
  library: *const c_char,
  member: *const c_char,
  constant: *mut CompileConstant,
) {
  luau_library_constant_lookup(cstr_to_str(library), cstr_to_str(member), constant);
}

/// # Safety
/// 回调契约：name 指向 name_length 字节的合法内存。
unsafe extern "C-unwind" fn userdata_remapper(
  _context: *mut c_void,
  name: *const c_char,
  name_length: usize,
) -> u8 {
  let bytes = unsafe { from_raw_parts(name.cast::<u8>(), name_length) };
  let name_str = from_utf8(bytes).unwrap_or("");
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

#[inline]
fn userdata_index_to_type(userdata_index: u8) -> u8 {
  LBC_TYPE_TAGGED_USERDATA_BASE + userdata_index
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
struct Vec2 {
  x: f32,
  y: f32,
}

impl Vec2 {
  const TAG: i32 = 12;
  const USERDATA_INDEX: u8 = UserdataKind::VEC2;
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

fn vector_access_bytecode_type(member: &str) -> u8 {
  const LBC_TYPE_NUMBER: u8 = 2;
  const LBC_TYPE_VECTOR: u8 = 8;
  const LBC_TYPE_ANY: u8 = 15;

  match member {
    "Magnitude" => LBC_TYPE_NUMBER,
    "Unit" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
}

fn vector_namecall_bytecode_type(member: &str) -> u8 {
  const LBC_TYPE_NUMBER: u8 = 2;
  const LBC_TYPE_VECTOR: u8 = 8;
  const LBC_TYPE_ANY: u8 = 15;

  match member {
    "Dot" => LBC_TYPE_NUMBER,
    "Cross" => LBC_TYPE_VECTOR,
    _ => LBC_TYPE_ANY,
  }
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

fn userdata_access_bytecode_type(ty: u8, member: &str) -> u8 {
  const LBC_TYPE_ANY: u8 = 0;
  const LBC_TYPE_NUMBER: u8 = 2;
  const LBC_TYPE_VECTOR: u8 = 8;

  match UserdataKind::from_type(ty) {
    Some(UserdataKind::Color) => match member {
      "R" | "G" | "B" => LBC_TYPE_NUMBER,
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Vec2) => match member {
      "X" | "Y" | "Magnitude" => LBC_TYPE_NUMBER,
      "Unit" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Mat3) => match member {
      "Row1" | "Row2" | "Row3" => LBC_TYPE_VECTOR,
      _ => LBC_TYPE_ANY,
    },
    Some(UserdataKind::Vertex) => match member {
      "pos" | "normal" => LBC_TYPE_VECTOR,
      "uv" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    _ => LBC_TYPE_ANY,
  }
}

fn userdata_metamethod_bytecode_type(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8 {
  const LBC_TYPE_ANY: u8 = 15;

  match method {
    HostMetamethod::Add | HostMetamethod::Sub | HostMetamethod::Mul | HostMetamethod::Div => {
      if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2)
        || UserdataKind::from_type(rhs_ty) == Some(UserdataKind::Vec2)
      {
        userdata_index_to_type(Vec2::USERDATA_INDEX)
      } else {
        LBC_TYPE_ANY
      }
    }
    HostMetamethod::Minus if UserdataKind::from_type(lhs_ty) == Some(UserdataKind::Vec2) => {
      userdata_index_to_type(Vec2::USERDATA_INDEX)
    }
    _ => LBC_TYPE_ANY,
  }
}

fn userdata_namecall_bytecode_type(ty: u8, member: &str) -> u8 {
  const LBC_TYPE_ANY: u8 = 0;
  const LBC_TYPE_NUMBER: u8 = 2;

  match UserdataKind::from_type(ty) {
    Some(UserdataKind::Vec2) => match member {
      "Dot" => LBC_TYPE_NUMBER,
      "Min" => userdata_index_to_type(Vec2::USERDATA_INDEX),
      _ => LBC_TYPE_ANY,
    },
    _ => LBC_TYPE_ANY,
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

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn vector_access_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = member_to_str(member, member_length);
  vector_access_bytecode_type(m)
}

/// # Safety
/// 回调契约：member 指向 member_length 字节的合法内存。
unsafe extern "C-unwind" fn vector_namecall_bytecode_type_callback(
  member: *const c_char,
  member_length: usize,
) -> u8 {
  let m = member_to_str(member, member_length);
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
  let m = member_to_str(member, member_length);
  unsafe { vector_access(&mut *builder, m, result_reg, source_reg, pcpos) }
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
  let m = member_to_str(member, member_length);
  unsafe {
    vector_namecall(
      &mut *builder,
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
  let m = member_to_str(member, member_length);
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
  let m = member_to_str(member, member_length);
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
  let m = member_to_str(member, member_length);
  unsafe { userdata_access(&mut *builder, ty, m, result_reg, source_reg, pcpos) }
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
  unsafe {
    userdata_metamethod(
      &mut *builder,
      lhs_ty,
      rhs_ty,
      result_reg,
      lhs,
      rhs,
      method,
      pcpos,
    )
  }
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
  let m = member_to_str(member, member_length);
  unsafe {
    userdata_namecall(
      &mut *builder,
      ty,
      m,
      arg_res_reg,
      source_reg,
      params,
      results,
      pcpos,
    )
  }
}

#[inline]
fn member_to_str(ptr: *const c_char, len: usize) -> &'static str {
  if ptr.is_null() || len == 0 {
    ""
  } else {
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

/// RAII 守卫：退出时关闭 lua_State（对应 C++ unique_ptr + lua_close）。
struct StateGuard(*mut lua_State);

impl StateGuard {
  fn new() -> Option<Self> {
    let l = lua_l_newstate();
    if l.is_null() { None } else { Some(Self(l)) }
  }

  fn as_ptr(&self) -> *mut lua_State {
    self.0
  }
}

impl Drop for StateGuard {
  fn drop(&mut self) {
    unsafe { lua_close(self.0) };
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
  options.vector_ctor = VECTOR_C;
  options.vector_type = VECTOR_C;
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
  fn initialize_codegen(&self, l: *mut lua_State) {
    if luau_codegen_supported() != 0 {
      // 类型 remapper 依赖 codegen 运行时
      // SAFETY: l 为新建且存活的 lua_State（对应 cpp C API 契约）
      unsafe {
        luau_codegen_create(l);
        set_userdata_remapper(l, null_mut(), userdata_remapper);
      }
    }
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
    let state = StateGuard::new().expect("lua state allocation failed");
    let l = state.as_ptr();

    self.initialize_codegen(l);

    let load_result = unsafe {
      luau_load(
        l,
        c"name".as_ptr(),
        bytecode.as_ptr().cast::<c_char>(),
        bytecode.len(),
        0,
      )
    };
    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(USERDATA_RUN_TYPES.as_ptr());
    let mut assembly_options = make_assembly_options(codegen_options, include_ir_types);
    assembly_options.compilation_options.flags = self.assembly_options.compilation_options.flags;
    assembly_options.include_outlined_code = self.assembly_options.include_outlined_code;
    assembly_options.include_reg_flow_info = self.assembly_options.include_reg_flow_info;

    // IR 断言用稳定 target
    let result =
      assembly_text(unsafe { get_assembly(l, -1, assembly_options.clone(), null_mut()) });

    if luau_codegen_supported() != 0 {
      // 同时验证另一 target 也能正确 lower
      assembly_options.target = Target::A64;
      unsafe { get_assembly(l, -1, assembly_options, null_mut()) };
    }

    if clip_to_first_return {
      clip_assembly_to_first_return(result)
    } else {
      result
    }
  }

  /// 经 C ABI `luau_compile` 编译源码后取 IR 文本。
  pub fn get_codegen_assembly_using_c_api(
    &mut self,
    source: &str,
    include_ir_types: bool,
    debug_level: i32,
  ) -> String {
    use alloc::ffi::CString;

    self.compilation_options_c.optimization_level = 2;
    self.compilation_options_c.debug_level = debug_level;
    self.compilation_options_c.type_info_level = 1;

    let mut compile_options = LuaCompileOptions {
      optimization_level: 2,
      debug_level,
      type_info_level: 1,
      coverage_level: 0,
      vector_lib: null(),
      vector_ctor: VECTOR_C,
      vector_type: VECTOR_C,
      mutable_globals: null(),
      userdata_types: USERDATA_COMPILE_TYPES.as_ptr(),
      libraries_with_known_members: LIBRARIES_WITH_CONSTANTS.as_ptr(),
      library_member_type_cb: Some(luau_library_type_lookup_callback),
      library_member_constant_cb: Some(luau_library_constant_lookup_callback),
      disabled_builtins: null(),
    };

    let c_source = CString::new(source).expect("source contains NUL");
    let mut bytecode_size = 0usize;
    let bytecode = unsafe {
      luau_compile(
        c_source.as_ptr(),
        source.len(),
        &mut compile_options,
        &mut bytecode_size,
      )
    };
    assert!(!bytecode.is_null());

    let state = StateGuard::new().expect("lua state allocation failed");
    let l = state.as_ptr();

    self.initialize_codegen(l);

    let load_result = unsafe { luau_load(l, c"name".as_ptr(), bytecode, bytecode_size, 0) };
    unsafe { libc_free(bytecode.cast()) };
    assert_eq!(load_result, 0, "Failed to load bytecode");

    let codegen_options = configure_codegen_options(USERDATA_RUN_TYPES.as_ptr());
    let assembly_options = make_assembly_options(codegen_options, include_ir_types);

    assembly_text(unsafe { get_assembly(l, -1, assembly_options, null_mut()) })
  }

  /// 取最后一个函数的头部（IR 类型注解），debugLevel 固定 2。
  pub fn get_codegen_header(&mut self, source: &str) -> String {
    let mut assembly = self.get_codegen_assembly(source, true, 2, 2, false);

    // 跳到最后一个函数
    while let Some(pos) = assembly[1..].find("; function ") {
      assembly = assembly[pos + 1..].to_string();
    }

    let bytecode_start = assembly
      .find("bb_bytecode_0:")
      .or_else(|| assembly.find("bb_0:"))
      .unwrap_or_else(|| panic!("bb_bytecode_0: not found"));

    assembly[..bytecode_start].to_string()
  }
}

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

#[inline]
unsafe fn libc_free(ptr: *mut c_void) {
  unsafe { free(ptr) }
}
