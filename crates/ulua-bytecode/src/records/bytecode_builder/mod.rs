use core::{
  fmt,
  fmt::{Debug, Formatter},
};
use std::{string::String, vec::Vec};

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_table::DenseEqDefault, instruction::Instruction,
  },
  type_aliases::dense_hash_fast::DenseHashFast,
};

use crate::{
  enums::{dump_flags::DumpFlags, r#type::Type},
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::{
    bytecode_encoder::Encoder, class_shape::ClassShape, constant::Constant,
    constant_key::ConstantKey, debug_local_bytecode_builder::DebugLocal, debug_upval::DebugUpval,
    function::Function, jump::Jump, string_ref::StringRef, table_shape::TableShape,
    typed_local::TypedLocal, typed_upval::TypedUpval, userdata_type::UserdataType,
  },
  type_aliases::string_table::StringTable,
};

// cpp 中 BytecodeBuilder 按值传递即浅拷贝（encoder 是裸指针）。Rust 端 encoder 已收敛为
// 内联 `Encoder`（无 Box<dyn>），故 Clone 可派生，语义与 cpp 拷贝构造一致。

/// 反汇编函数指针：`set_dump_flags` 装上 `dump_current_function` 后，`end_function`
/// 逐函数调用它，取回 dump 文本与「指令 → 文本偏移」表。
type DumpFn<'a> = for<'b> fn(&'b BytecodeBuilder<'a>) -> (String, Vec<i32>);

/// `kMaxConstantCount = 1 << 23`（cpp BytecodeBuilder.h:20）。
pub(crate) const K_MAX_CONSTANT_COUNT: u32 = 1 << 23;
/// `kMaxClosureCount = 1 << 15`（cpp BytecodeBuilder.h:21）。
pub(crate) const K_MAX_CLOSURE_COUNT: u32 = 1 << 15;
/// `kMaxUpvalueCount = 200`（cpp BytecodeBuilder.h:14）。
pub(crate) const K_MAX_UPVALUE_COUNT: u32 = 200;
/// `kInvalidReg = 255`（cpp BytecodeBuilder.h:18）。
pub(crate) const K_INVALID_REG: u32 = 255;
/// `kMaxJumpDistance = 1 << 23`（cpp BytecodeBuilder.h:23）：JUMPX 24 位偏移上限。
pub(crate) const K_MAX_JUMP_DISTANCE: i32 = 1 << 23;
// `LOP_GETIMPORT` aux/import id 段布局常量（高 2 位组件数、每组件 10 位常量索引、
// 组件移位量）的单一权威定义在 `ulua_common::functions::import_layout`；解码入口
// `decode_import_aux`、`rebuild_graph`、`validate_instructions` 的 GETIMPORT 分支与
// 编码入口 `get_import_id*`、`emit_instruction` 的 GETIMPORT 一律直接从该处取用。

/// 指令字编码布局（cpp `Luau/Bytecode.h` 的 `LUAU_INSN_*` 宏族，与
/// `ulua_common::macros::luau_insn_*` 读取侧互为镜像）。
///
/// ```text
/// 31          24 23          16 15           8 7            0
/// ┌──────────────┬──────────────┬──────────────┬──────────────┐
/// │      C       │      B       │      A       │     OP       │
/// └──────────────┴──────────────┴──────────────┴──────────────┘
/// ```
pub(crate) mod insn {
  /// OP 域掩码：与 `Instruction::opcode()` 读取侧同值。
  pub(crate) const OP_MASK: u32 = 0xff;
  /// A 域移位。
  pub(crate) const A_SHIFT: u32 = 8;
  /// B / D 域移位（D 为 A 之后的 16 位有偏移域）。
  pub(crate) const B_SHIFT: u32 = 16;
  /// D 域掩码（低 16 位之外全清零，cpp 的 `*insn &= 0xffff`）。
  pub(crate) const AD_MASK: u32 = 0xffff;
  /// C 域移位。
  pub(crate) const C_SHIFT: u32 = 24;

  /// JUMPXEQKN/S 的 aux 字最高位：条件取反（NOT）标记。
  pub(crate) const AUX_INVERT_BIT: u32 = 1 << 31;
  /// JUMPXEQKB 的 aux 字最低位：被比较的布尔常量值（1 即 true）。
  pub(crate) const AUX_BOOL_VALUE_BIT: u32 = 1 << 0;
  /// UDATA KS 族（GETUDATAKS/SETUDATAKS/NAMECALLUDATA）aux 字的标志域移位：
  /// 低 16 位为 Aux16 常量索引（`luau_insn_aux_kv16`），上 16 位为 imm 标志。
  pub(crate) const AUX_FLAGS_SHIFT: u32 = 16;
  /// aux 字第二个字节子域的移位（FASTCALL3 第三寄存器）。
  pub(crate) const AUX_BYTE_SHIFT: u32 = 8;

  /// aux 字标志位的唯一编码源：`on` 时落 `bit`，否则为 0（JUMPXEQK* 的 NOT 位、
  /// 布尔常量值位共用的 `if b { BIT } else { 0 }` 形态收口于此）。
  pub(crate) const fn bit_if(on: bool, bit: u32) -> u32 {
    if on { bit } else { 0 }
  }

  /// dump 依 aux 的 NOT 位（`AUX_INVERT_BIT`）给出的 " NOT" 后缀：
  /// 该字面量在全仓仅出现于此，四处展示臂共用。
  pub(crate) const fn invert_label(aux: u32) -> &'static str {
    if aux & AUX_INVERT_BIT != 0 {
      " NOT"
    } else {
      ""
    }
  }
}

impl Debug for BytecodeBuilder<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("BytecodeBuilder").finish_non_exhaustive()
  }
}

/// `'a` 是「调用方借给本 builder 的字符串寿命」：cpp 里 `stringTable` / `debugStrings` /
/// `userdataTypes[].name` 都是指向编译器 AST 名表的 `StringRef`（不拷贝），Rust 用同一个
/// 生命周期参数把这条不变量化，避免裸指针。
#[derive(Clone)]
pub struct BytecodeBuilder<'a> {
  pub(crate) functions: Vec<Function>,
  /// cpp `uint32_t currentFunction = ~0u`：`None` 即 cpp 的 `~0u` 哨兵，表示
  /// 当前没有进行中的函数体（begin/end 之间的状态位），用 Option 让「未开始」
  /// 在类型上可表示，散点的 `== u32::MAX` 判断随之消失。
  pub(crate) current_function: Option<u32>,
  pub(crate) main_function: u32,

  pub total_instruction_count: usize,
  pub(crate) insns: Vec<u32>,
  pub(crate) lines: Vec<i32>,
  pub(crate) constants: Vec<Constant>,
  pub(crate) protos: Vec<u32>,
  pub(crate) jumps: Vec<Jump>,

  pub(crate) table_shapes: Vec<TableShape>,
  pub(crate) class_shapes: Vec<ClassShape>,

  pub(crate) fb_slots: Vec<u32>,

  pub(crate) has_long_jumps: bool,

  // 两张去重缓存只做 find / try_insert / clear，从不遍历（常量与 table shape 最终
  // 由 `constants` / `table_shapes` 这两个 Vec 按插入序落盘），迭代序不构成契约，
  // 故换 foldhash 快版哈希器（见 `ulua_common::type_aliases::dense_hash_fast`）。
  pub(crate) constant_map:
    DenseHashMap<ConstantKey, i32, DenseHashFast<ConstantKey>, DenseEqDefault<ConstantKey>>,
  pub(crate) table_shape_map:
    DenseHashMap<TableShape, i32, DenseHashFast<TableShape>, DenseEqDefault<TableShape>>,
  pub(crate) proto_map: DenseHashMap<u32, i16>,

  pub(crate) debug_line: i32,

  pub(crate) debug_locals: Vec<DebugLocal>,
  pub(crate) debug_upvals: Vec<DebugUpval>,

  pub(crate) typed_locals: Vec<TypedLocal>,
  pub(crate) typed_upvals: Vec<TypedUpval>,

  pub(crate) userdata_types: Vec<UserdataType<'a>>,

  // 字符串表要按插入遍历序落盘，类型（含 FNV 版哈希器与 null 空槽哨兵）见 `StringTable`。
  pub(crate) string_table: StringTable<'a>,
  pub(crate) debug_strings: Vec<StringRef<'a>>,

  pub(crate) debug_remarks: Vec<(u32, u32)>,
  pub(crate) debug_remark_buffer: String,

  // cpp 里是 `BytecodeEncoder*` 裸指针（nullptr = 不做变换）。全仓唯一实现是 `NoopEncoder`，
  // 故用内联封闭 enum 取代 `Box<dyn BytecodeEncoder>`：无堆分配、无 vtable 间接调用，
  // `end_function` 里的 match 静态分发可完全内联。
  pub(crate) encoder: Option<Encoder>,
  // cpp `std::string bytecode`：原始字节缓冲（非 UTF-8），Rust 端用 Vec<u8> 忠实建模。
  pub(crate) bytecode: Vec<u8>,

  pub(crate) dump_flags: u32,
  pub(crate) dump_source: Vec<String>,
  pub(crate) dump_remarks: Vec<(i32, String)>,

  // cpp `std::string tempTypeInfo`：类型信息序列化字节缓冲。
  pub(crate) temp_type_info: Vec<u8>,

  /// 置位后 `end_function` 会调用它反汇编当前函数，取回 dump 文本与指令偏移表。
  pub(crate) dump_function_ptr: Option<DumpFn<'a>>,
}

impl<'a> BytecodeBuilder<'a> {
  pub fn get_string_hash(key: StringRef<'a>) -> u32 {
    bytecode_builder_get_string_hash(key)
  }

  pub const DUMP_CODE: u32 = DumpFlags::Code as u32;
  pub const DUMP_LINES: u32 = DumpFlags::Lines as u32;
  pub const DUMP_SOURCE: u32 = DumpFlags::Source as u32;
  pub const DUMP_LOCALS: u32 = DumpFlags::Locals as u32;
  pub const DUMP_REMARKS: u32 = DumpFlags::Remarks as u32;
  pub const DUMP_TYPES: u32 = DumpFlags::Types as u32;
  pub const DUMP_CONSTANTS: u32 = DumpFlags::Constants as u32;

  /// 获取当前待决指令流的强类型切片视图。
  #[inline(always)]
  pub fn instructions(&self) -> &[Instruction] {
    Instruction::from_slice(&self.insns)
  }
}

impl Default for BytecodeBuilder<'_> {
  /// 委托 [`Self::new`]，确保 `constant_map` 的空键哨兵与 `{Nil, 0, 0}` 这条
  /// 真实的 nil 常量键不相撞（否则 nil 永不去重、`insert_unsafe` 虚增计数）。
  fn default() -> Self {
    Self::new(None)
  }
}

mod constants;
mod dump;
mod emit;
mod function;
mod interning;
mod jumps;
mod scan;
mod serialize;
mod validate;

// ── abs-r139：并自 `methods/bytecode_builder_bytecode_builder.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn new(encoder: Option<Encoder>) -> Self {
    let constant_map = DenseHashMap::new(ConstantKey {
      r#type: Type::Nil,
      value: !0u64,
      extra: 0,
      extra2: 0,
      extra3: 0,
    });

    // 注意：`TableShape::default()`（零长度 DUPTABLE 可达）不可作哨兵，
    // 详见 `TableShape::EMPTY_KEY_SENTINEL` 的说明。
    let table_shape_map = DenseHashMap::new(TableShape::EMPTY_KEY_SENTINEL);

    let proto_map = DenseHashMap::new(!0u32);

    let string_table = DenseHashMap::new(StringRef::default());

    let mut result = BytecodeBuilder {
      functions: Vec::new(),
      current_function: None,
      main_function: !0u32,
      total_instruction_count: 0,
      insns: Vec::new(),
      lines: Vec::new(),
      constants: Vec::new(),
      protos: Vec::new(),
      jumps: Vec::new(),
      table_shapes: Vec::new(),
      class_shapes: Vec::new(),
      fb_slots: Vec::new(),
      has_long_jumps: false,
      constant_map,
      table_shape_map,
      proto_map,
      debug_line: 0,
      debug_locals: Vec::new(),
      debug_upvals: Vec::new(),
      typed_locals: Vec::new(),
      typed_upvals: Vec::new(),
      userdata_types: Vec::new(),
      string_table,
      debug_strings: Vec::new(),
      debug_remarks: Vec::new(),
      debug_remark_buffer: String::new(),
      encoder,
      bytecode: Vec::new(),
      dump_flags: 0,
      dump_source: Vec::new(),
      dump_remarks: Vec::new(),
      temp_type_info: Vec::new(),
      dump_function_ptr: None,
    };

    // preallocate some buffers that are very likely to grow anyway; this works around std::vector's inefficient growth policy for small arrays
    result.insns.reserve(32);
    result.lines.reserve(32);
    result.constants.reserve(16);
    result.protos.reserve(16);
    result.functions.reserve(8);

    // LUAU_ASSERT(stringTable.find(StringRef{"", 0}) == nullptr);
    let empty_key = StringRef::from_slice(b"");
    LUAU_ASSERT!(result.string_table.find(&empty_key).is_none());

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_debug_pc.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_debug_pc(&self) -> u32 {
    self.insns.len() as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_function_count.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp: `BytecodeBuilder::getFunctionCount`
  /// (`cpp/Bytecode/include/Luau/BytecodeBuilder.h:173`)
  pub fn get_function_count(&self) -> u32 {
    self.functions.len() as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_function_data.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_function_data(&self, id: u32) -> Vec<u8> {
    self.functions[id as usize].data.clone()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_instruction_count.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_instruction_count(&self) -> usize {
    self.insns.len()
  }
}
