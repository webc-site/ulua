use core::{
  cmp::min,
  fmt,
  fmt::{Arguments, Debug, Formatter},
  iter::repeat_n,
  mem,
  mem::take,
  slice,
};
use std::{string::String, vec, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_tag::LuauBytecodeTag, luau_bytecode_type::LuauBytecodeType,
    luau_capture_type::LuauCaptureType, luau_feedback_type::LuauFeedbackType,
    luau_opcode::LuauOpcode, luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  fflag::DebugLuauUserDefinedClasses,
  functions::{
    format_append::format_append,
    format_g::{format_g, format_g_append_vector},
    get_jump_target::get_jump_target,
    get_op_length::get_op_length,
    import_layout::{K_IMPORT_COMPONENT_MASK, K_IMPORT_COUNT_SHIFT, import_component_shift},
    is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call,
    is_jump_d::is_jump_d,
    is_skip_c::is_skip_c,
  },
  macros::{luau_assert::LUAU_ASSERT, luau_assertenabled::LUAU_ASSERTENABLED},
  records::{
    dense_hash_map::DenseHashMap, dense_hash_table::DenseEqDefault, instruction::Instruction,
    small_vector::SmallVector,
  },
  type_aliases::dense_hash_fast::DenseHashFast,
};

use crate::{
  enums::{dump_flags::DumpFlags, r#type::Type},
  functions::{
    bytecode_write::{write_byte, write_bytes, write_var_int},
    decode_import_aux::decode_import_aux,
    get_base_type_string::get_base_type_string,
    log_2::{ceillog2, log2},
    printable_string_constant::printable_string_constant,
  },
  macros::{
    vconst::VCONST, vconstany::VCONSTANY, vjump::VJUMP, vreg::VREG, vregrange::VREGRANGE,
    vupval::VUPVAL,
  },
  methods::bytecode_builder_get_string_hash::{
    bytecode_builder_get_string_hash, bytecode_builder_get_string_hash_slice,
  },
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

  pub(crate) total_instruction_count: usize,
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

// ── abs-r139：并自 `methods/bytecode_builder_add_child_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_child_function(&mut self, fid: u32) -> i16 {
    if let Some(cache) = self.proto_map.find(&fid) {
      return *cache;
    }

    let id = self.protos.len() as u32;

    if id >= K_MAX_CLOSURE_COUNT {
      return -1;
    }

    self.proto_map.try_insert(fid, id as i16);
    self.protos.push(fid);

    id as i16
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_class_shape.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_class_shape(&mut self, shape: ClassShape) -> i32 {
    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    let c = Constant::ClassShape(self.class_shapes.len() as u32);

    self.class_shapes.push(shape);
    self.constants.push(c);

    id as i32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_constant.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 常量去重表入口：缓存键由常量本身推导（`Constant::key`），调用点不再手搭。
  pub(crate) fn add_constant(&mut self, value: Constant) -> i32 {
    let key = value.key();
    if let Some(cache) = self.constant_map.find(&key) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    self.constant_map.try_insert(key, id as i32);
    self.constants.push(value);

    id as i32
  }

  pub fn add_constant_nil(&mut self) -> i32 {
    self.add_constant(Constant::Nil)
  }

  pub fn add_constant_boolean(&mut self, value: bool) -> i32 {
    self.add_constant(Constant::Boolean(value))
  }

  pub fn add_constant_number(&mut self, value: f64) -> i32 {
    self.add_constant(Constant::Number(value))
  }

  pub fn add_constant_integer(&mut self, value: i64) -> i32 {
    self.add_constant(Constant::Integer(value))
  }

  pub fn add_constant_string(&mut self, value: StringRef<'a>) -> i32 {
    let index = self.add_string_table_entry(value);
    self.add_constant(Constant::String(index))
  }

  /// cpp `addConstantVectorf`：键打包布局（x/y 进 value、z/w 进 extra）收敛在
  /// `Constant::key`，此处只构造常量本体。
  pub fn add_constant_vector(&mut self, x: f32, y: f32, z: f32, w: f32) -> i32 {
    self.add_constant(Constant::Vector([x, y, z, w]))
  }

  /// cpp `addConstantVectord`：四分量键打包布局收敛在 `Constant::key`。
  pub fn add_constant_vector_d(&mut self, x: f64, y: f64, z: f64, w: f64) -> i32 {
    self.add_constant(Constant::Vectord([x, y, z, w]))
  }

  pub fn add_constant_closure(&mut self, fid: u32) -> i32 {
    self.add_constant(Constant::Closure(fid))
  }

  pub fn add_constant_table(&mut self, shape: &TableShape) -> i32 {
    // 去重键不得撞上 `DenseHashMap` 的空键哨兵：撞上时 `find` 恒 `None`、
    // `try_insert` 会把哨兵写进槽位，故在入口把这条不变量显式化。
    debug_assert_ne!(
      *shape,
      TableShape::EMPTY_KEY_SENTINEL,
      "TableShape 去重键不得与 DenseHashMap 空键哨兵相撞"
    );

    if let Some(cache) = self.table_shape_map.find(shape) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    // C++ `value.valueTable = uint32_t(tableShapes.size())`: Table 载荷
    // indexes table_shapes, NOT constants. The previous `id` (= constants
    // length) over-indexed table_shapes and panicked in write_function.
    let value = Constant::Table(self.table_shapes.len() as u32);

    self.table_shape_map.try_insert(*shape, id as i32);
    self.table_shapes.push(*shape);
    self.constants.push(value);

    id as i32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_debug_remark.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_debug_remark(&mut self, args: Arguments<'_>) {
    if !DumpFlags::Remarks.is_set(self.dump_flags) {
      return;
    }

    let offset = self.debug_remark_buffer.len();

    // C++ `addDebugRemark(const char* format, ...)` printf-formats the whole remark.
    // Rust has no printf, so callers pass the fully-rendered message as a single
    // `format_args!(...)` (the C-style `%d`/`%.2f` become `{}`/`{:.2}`).
    format_append(&mut self.debug_remark_buffer, args);

    // we null-terminate all remarks to avoid storing remark length
    self.debug_remark_buffer.push('\0');

    self
      .debug_remarks
      .push((self.insns.len() as u32, offset as u32));

    let bytes = &self.debug_remark_buffer.as_bytes()[offset..];
    let remark_len = memchr::memchr(0, bytes).unwrap_or(bytes.len()) as i32;

    let remark_str = self.debug_remark_buffer[offset..offset + (remark_len as usize)].to_string();

    self.dump_remarks.push((self.debug_line, remark_str));
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_fb_slot.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_fb_slot(&mut self, t: LuauFeedbackType) -> u32 {
    LUAU_ASSERT!(t == LuauFeedbackType::LFT_CALLTARGET);
    self.fb_slots.push(self.get_instruction_count() as u32);
    (self.fb_slots.len() - 1) as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_import.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_import(&mut self, iid: u32) -> i32 {
    self.add_constant(Constant::Import(iid))
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_string_table_entry.rs` ──
/// cpp `BytecodeBuilder::addStringTableEntry`（`Bytecode/src/BytecodeBuilder.cpp:319`）的实体。
///
/// 拆成自由函数是因为 `finalize` 需要在遍历 `userdataTypes` 的同时写表：把 `string_table` /
/// `debug_strings` / `dump_flags` 作为分离字段借出后即可避开 `&mut self` 与字段借用的冲突，
/// 不必为绕开借用检查而退回裸指针或索引循环。
pub(crate) fn bytecode_builder_intern_string<'a>(
  string_table: &mut StringTable<'a>,
  debug_strings: &mut Vec<StringRef<'a>>,
  dump_flags: u32,
  value: StringRef<'a>,
) -> u32 {
  if let Some(idx) = string_table.find(&value) {
    return *idx;
  }

  // Bytecode serialization uses 1-based string-table indices (0 is reserved
  // to mean "no string"). C++ computes the index as `stringTable.size()`
  // *after* inserting the new entry via `operator[]`, i.e. pre-insert size+1.
  // Computing it pre-insert made the first string index 0, so
  // `debugStrings[valueString - 1]` underflowed in `dumpConstant`.
  let new_index = string_table.size() as u32 + 1;
  string_table.try_insert(value.clone(), new_index);

  if DumpFlags::Code.is_set(dump_flags) {
    debug_strings.push(value);
  }

  new_index
}

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn add_string_table_entry(&mut self, value: StringRef<'a>) -> u32 {
    bytecode_builder_intern_string(
      &mut self.string_table,
      &mut self.debug_strings,
      self.dump_flags,
      value,
    )
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_userdata_type.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp `addUserDataTypes`：只登记名字**视图**（不拷贝），名字缓冲必须活得比 builder 久，
  /// 这条不变量由 `BytecodeBuilder<'a>`/`StringRef<'a>` 静态保证。
  pub fn add_userdata_type(&mut self, name: StringRef<'a>) -> u32 {
    let ty = UserdataType {
      name,
      ..Default::default()
    };

    self.userdata_types.push(ty);
    (self.userdata_types.len() - 1) as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_annotate_instruction.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn annotate_instruction(&self, result: &mut String, fid: u32, instpos: u32) {
    if !DumpFlags::Code.is_set(self.dump_flags) {
      return;
    }

    LUAU_ASSERT!(fid < self.functions.len() as u32);

    let function = &self.functions[fid as usize];
    let (dump, dumpinstoffs) = (&function.dump, &function.dumpinstoffs);

    let next = instpos + 1;

    LUAU_ASSERT!(next < dumpinstoffs.len() as u32);

    // Skip locations of multi-dword instructions（定位下一个非 -1 偏移）
    let next = dumpinstoffs[instpos as usize + 1..]
      .iter()
      .position(|&off| off != -1)
      .map_or(dumpinstoffs.len() as u32, |k| instpos + 1 + k as u32);

    let start_offset = dumpinstoffs[instpos as usize] as usize;
    let end_offset = dumpinstoffs[next as usize] as usize;

    // cpp `formatAppend(result, "%.*s", len, dump.data() + start)`：定长字节追加，
    // Rust 直接 push_str，免过格式化机
    result.push_str(&dump[start_offset..end_offset]);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_begin_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn begin_function(&mut self, numparams: u8, isvararg: bool) -> u32 {
    LUAU_ASSERT!(self.current_function.is_none());

    let id = self.functions.len() as u32;

    let func = Function {
      numparams,
      isvararg,
      ..Default::default()
    };

    self.functions.push(func);

    self.current_function = Some(id);

    self.has_long_jumps = false;
    self.debug_line = 0;

    id
  }
}

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

// ── abs-r139：并自 `methods/bytecode_builder_clear_strings.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp: `BytecodeBuilder::clearStrings`
  /// (`cpp/Bytecode/include/Luau/BytecodeBuilder.h:182`)
  pub fn clear_strings(&mut self) {
    self.debug_strings.clear();
    self.string_table.clear();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_decompose_import_id.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 元组返回 `(count, id0, id1, id2)` 替代 cpp 的三个 `int32_t&` 出参。
  /// 与 aux 解码同用 `K_IMPORT_*` 布局常量与 `import_component_shift`（import id 与 aux 同构打包）。
  pub(crate) fn decompose_import_id(ids: u32) -> (i32, i32, i32, i32) {
    let count = (ids >> K_IMPORT_COUNT_SHIFT) as i32;
    let component =
      |k: u32| (ids >> import_component_shift(k)) as i32 & K_IMPORT_COMPONENT_MASK as i32;
    let id0 = if count > 0 { component(0) } else { -1 };
    let id1 = if count > 1 { component(1) } else { -1 };
    let id2 = if count > 2 { component(2) } else { -1 };
    (count, id0, id1, id2)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_constant.rs` ──
/// 常量串 dump 的截断长度（超出部分以 `...` 省略）。
const K_MAX_DUMPED_STRING_LEN: usize = 32;

impl<'a> BytecodeBuilder<'a> {
  /// 取 import 段的字符串常量。误用防护：`idx` 指向的常量在构建时已注册为
  /// String 且索引非 0（入口断言兜底）。
  fn import_segment_str(&self, idx: i32) -> &StringRef<'a> {
    let str_idx = self.constants[idx as usize].as_string();
    LUAU_ASSERT!(str_idx as usize <= self.debug_strings.len());
    &self.debug_strings[str_idx as usize - 1]
  }

  pub(crate) fn dump_constant(&self, result: &mut String, k: i32, detailed: bool) {
    LUAU_ASSERT!((k as u32) < self.constants.len() as u32);
    let data = &self.constants[k as usize];

    match data {
      Constant::Nil => result.push_str("nil"),
      Constant::Boolean(value) => result.push_str(if *value { "true" } else { "false" }),
      Constant::Number(value) => result.push_str(&format_g(*value, 17)),
      Constant::Integer(value) => format_append(result, format_args!("{value}")),
      // cpp `BytecodeBuilder.cpp` 两分支同构：`%.9g` 打 float 分量，截断判定
      // `v[3] == 0`（f32→f64 提升精确，与原 f32 判定等价）
      Constant::Vector(v) => {
        format_g_append_vector(result, &v.map(f64::from), |x| format_g(x, 9));
      }
      // cpp `BytecodeBuilder.cpp:2353-2377`：flag 开时按 %.17g 打双精度，
      // 关时先转 float 再按 %.9g 打（与 writeFunction 的降级路径一致）。
      // 三分量截断只看原始 `v[3] == 0`（helper 内先判后打，变换仅作用于
      // 打印值），与 flag 无关。
      Constant::Vectord(v) => {
        let wide = fflag::LuauCompileEmitVectorDouble.get();
        format_g_append_vector(result, v, |x| {
          format_g(
            if wide { x } else { f64::from(x as f32) },
            if wide { 17 } else { 9 },
          )
        });
      }
      Constant::String(str_idx) => {
        let str = &self.debug_strings[*str_idx as usize - 1];
        let bytes = str.as_bytes();
        if printable_string_constant(bytes) {
          // 显示路径用 lossy 容错；printable 分支均为合法 UTF-8，输出无损
          let s = str.to_string_lossy();
          let limit = min(s.len(), K_MAX_DUMPED_STRING_LEN);
          format_append(result, format_args!("'{:.*}'", limit, s));
          if str.len() >= K_MAX_DUMPED_STRING_LEN {
            result.push_str("...");
          }
        } else {
          result.push('\'');
          for &b in &bytes[..min(str.len(), K_MAX_DUMPED_STRING_LEN)] {
            if b < b' ' {
              format_append(result, format_args!("\\x{:02X}", b));
            } else {
              result.push(b as char);
            }
          }
          if str.len() >= K_MAX_DUMPED_STRING_LEN {
            result.push_str("'...");
          } else {
            result.push('\'');
          }
        }
      }
      Constant::Import(iid) => {
        let (count, id0, id1, id2) = BytecodeBuilder::decompose_import_id(*iid);
        for (i, &id) in [id0, id1, id2][..count as usize].iter().enumerate() {
          if i > 0 {
            result.push('.');
          }
          result.push_str(&self.import_segment_str(id).to_string_lossy());
        }
      }
      Constant::Table(shape_idx) => {
        if detailed {
          let shape = &self.table_shapes[*shape_idx as usize];
          let sizenode = if shape.length > 0 {
            1u32 << (ceillog2(shape.length as i32) as u32)
          } else {
            0u32
          };
          let mask = if sizenode > 0 { sizenode - 1 } else { 0u32 };

          let mut slots = vec![0u32; shape.length as usize];
          let mut slot_owner = vec![!0u32; sizenode as usize];

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            let key_const = &self.constants[key_idx as usize];
            LUAU_ASSERT!(key_const.kind() == Type::String && key_const.as_string() != 0);
            let str = &self.debug_strings[key_const.as_string() as usize - 1];
            let hash = bytecode_builder_get_string_hash_slice(str.as_bytes());
            slots[i] = hash & mask;

            if slot_owner[slots[i] as usize] == !0u32 {
              slot_owner[slots[i] as usize] = i as u32;
            }
          }

          result.push('{');

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            if i > 0 {
              result.push_str(", ");
            }

            result.push('[');
            self.dump_constant(result, key_idx, false);
            result.push(']');

            if shape.has_constants && shape.constants[i] != -1 {
              result.push_str(" = ");
              self.dump_constant(result, shape.constants[i], false);
            }

            format_append(result, format_args!(" #{}", slots[i]));

            if slot_owner[slots[i] as usize] != i as u32 {
              result.push_str(" (conflict)");
            }
          }

          format_append(result, format_args!("}} sizenode={sizenode}"));
        } else {
          result.push_str("{...}");
        }
      }
      Constant::Closure(fid) => {
        let func = &self.functions[*fid as usize];
        // detailed 模式下输出 "function"/"function <名>"，非 detailed 输出 "'<名>'"
        if detailed {
          result.push_str("function");
          if !func.dumpname.is_empty() {
            result.push(' ');
            result.push_str(&func.dumpname);
          }
        } else if !func.dumpname.is_empty() {
          format_append(result, format_args!("'{}'", func.dumpname));
        }
      }
      Constant::ClassShape(shape_idx) => {
        let cs = &self.class_shapes[*shape_idx as usize];
        let class_name_const = &self.constants[cs.class_name as usize];
        LUAU_ASSERT!(
          class_name_const.kind() == Type::String
            && class_name_const.as_string() as usize <= self.debug_strings.len()
        );
        let str = &self.debug_strings[class_name_const.as_string() as usize - 1];
        LUAU_ASSERT!(printable_string_constant(str.as_bytes()));
        format_append(
          result,
          format_args!(
            "class {} (props: {}, methods: {})",
            str.to_string_lossy(),
            cs.property_names.len(),
            cs.method_names.len()
          ),
        );

        // detailed 模式追加 props/methods 的常量索引清单（两族同构，单源迭代）
        if detailed {
          for (label, keys) in [("props", &cs.property_names), ("methods", &cs.method_names)] {
            if keys.is_empty() {
              continue;
            }
            format_append(result, format_args!("\n  {label}:"));
            for &k in keys {
              format_append(result, format_args!("\n    K{} [", k));
              self.dump_constant(result, k, false);
              result.push(']');
            }
          }
        }
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_current_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 类型名 + 可选 '?' 后缀（userdata 名优先，回落到基础类型名）。
  /// C++ `name = userdata ? userdata : getBaseTypeString(et)`。
  fn type_name_and_optional(&self, ty: LuauBytecodeType) -> (&str, &'static str) {
    let name = self
      .try_get_userdata_type_name(ty)
      .unwrap_or_else(|| get_base_type_string(ty.0 as u8));
    let optional = if (ty.0 & LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0) != 0 {
      "?"
    } else {
      ""
    };
    (name, optional)
  }

  /// 反汇编当前函数：返回 `(dump 文本, 每条指令的文本起始偏移表)`。
  ///
  /// 偏移表长 `insns.len() + 1`（末项为文本总长），非 Code 段填充 `-1`；
  /// 由 `end_function` 写回所属 `Function`，供 `annotate_instruction` 定位指令文本区间。
  pub(crate) fn dump_current_function(&self) -> (String, Vec<i32>) {
    // 只读 dump 开关下的空产出：偏移表在 Code 分支内按需重建，此处给空表。
    if (self.dump_flags & (DumpFlags::Code | DumpFlags::Constants)) == 0 {
      return (String::new(), Vec::new());
    }

    let mut dumpinstoffs: Vec<i32> = Vec::new();

    let mut last_line = -1;
    let mut result = String::new();

    if DumpFlags::Locals.is_set(self.dump_flags) {
      for (i, l) in self.debug_locals.iter().enumerate() {
        if l.startpc == l.endpc {
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, no live range\n",
              i, l.reg, l.startpc, self.lines[l.startpc as usize]
            ),
          );
        } else {
          LUAU_ASSERT!(l.startpc < l.endpc);
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);
          LUAU_ASSERT!(l.endpc <= self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, end pc {} line {}\n",
              i,
              l.reg,
              l.startpc,
              self.lines[l.startpc as usize],
              l.endpc - 1,
              self.lines[(l.endpc - 1) as usize]
            ),
          );
        }
      }
    }

    if DumpFlags::Types.is_set(self.dump_flags) {
      // cpp `functions.back().typeinfo`：空表时 release 下是 UB。dump 属调试通道，
      // 按断言 + 非 panic 读法收敛：无函数时视作空类型信息，不再解包 panic。
      LUAU_ASSERT!(!self.functions.is_empty());
      static NO_TYPEINFO: Vec<u8> = Vec::new();
      let typeinfo_bytes = self
        .functions
        .last()
        .map_or(&NO_TYPEINFO[..], |f| &f.typeinfo);

      if typeinfo_bytes.len() >= 2 {
        for (i, &et) in typeinfo_bytes[2..].iter().enumerate() {
          let (name, optional) = self.type_name_and_optional(LuauBytecodeType(et as u16));
          format_append(
            &mut result,
            format_args!("R{}: {}{} [argument]\n", i, name, optional),
          );
        }
      }

      for (i, l) in self.typed_upvals.iter().enumerate() {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(&mut result, format_args!("U{}: {}{}\n", i, name, optional));
      }

      for l in &self.typed_locals {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(
          &mut result,
          format_args!(
            "R{}: {}{} from {} to {}\n",
            l.reg, name, optional, l.startpc, l.endpc
          ),
        );
      }
    }

    if DumpFlags::Constants.is_set(self.dump_flags) {
      for (i, _) in self.constants.iter().enumerate() {
        format_append(&mut result, format_args!("K{}: ", i));
        self.dump_constant(&mut result, i as i32, true);
        result.push('\n');
      }
    }

    if DumpFlags::Code.is_set(self.dump_flags) {
      let insns_slice = self.instructions();
      let mut labels = vec![-1; insns_slice.len()];

      // 第一遍：标记跳转目标槽位（变步长遍历）
      let mut insns = insns_slice.iter().copied().enumerate();
      while let Some((i, insn)) = insns.next() {
        let target = get_jump_target(insn.raw(), i as u32);

        if target >= 0 {
          LUAU_ASSERT!((target as usize) < insns_slice.len());
          labels[target as usize] = 0;
        }

        let op = insn.luau_opcode();
        // 变步长推进：等价 cpp `i += getOpLength(op)`
        for _ in 1..get_op_length(op) as usize {
          insns.next();
        }
      }

      let mut next_label = 0;

      for label in &mut labels {
        if *label == 0 {
          *label = next_label;
          next_label += 1;
        }
      }

      dumpinstoffs.resize(insns_slice.len() + 1, -1);

      let mut remarks = self.debug_remarks.iter().copied().peekable();

      let mut insns = insns_slice.iter().copied().enumerate();
      while let Some((i, insn)) = insns.next() {
        dumpinstoffs[i] = result.len() as i32;

        // C++: `if (op == LOP_PREPVARARGS) { i++; continue; }` — the vararg
        // prologue is a call-dispatch Header with no "interesting" info and
        // is never disassembled. (The prior `op == 32` was a mistranslated
        // literal; LOP_PREPVARARGS is 65, so the skip never fired and the
        // Header reached `dump_instruction`'s unsupported-opcode assert.)
        if insn.opcode() == Some(LuauOpcode::LOP_PREPVARARGS) {
          continue;
        }

        if DumpFlags::Remarks.is_set(self.dump_flags) {
          // 归并输出起始于当前指令位置的 remark
          while let Some((_, remark_start)) = remarks.next_if(|(pos, _)| *pos == i as u32) {
            // C++ reads `debugRemarkBuffer.c_str() + offset` — a C-string that stops
            // at the null terminator. remark_end points at the *next* remark (past this
            // remark's '\0'), so slice only up to the terminator.
            let remark_end = remarks
              .peek()
              .map_or(self.debug_remark_buffer.len(), |&(_, end)| end as usize);
            let remark_str = &self.debug_remark_buffer[remark_start as usize..remark_end];
            let remark_str = remark_str.split('\0').next().unwrap_or("");
            format_append(&mut result, format_args!("REMARK {}\n", remark_str));
          }
        }

        if DumpFlags::Source.is_set(self.dump_flags) {
          let line = self.lines[i];

          if line > 0 && line != last_line {
            LUAU_ASSERT!(((line - 1) as usize) < self.dump_source.len());
            format_append(
              &mut result,
              format_args!("{:5}: {}\n", line, self.dump_source[(line - 1) as usize]),
            );
            last_line = line;
          }
        }

        if DumpFlags::Lines.is_set(self.dump_flags) {
          format_append(&mut result, format_args!("{}: ", self.lines[i]));
        }

        if labels[i] != -1 {
          format_append(&mut result, format_args!("L{}: ", labels[i]));
        }

        let target = get_jump_target(insn.raw(), i as u32);
        let target_label = if target >= 0 {
          labels[target as usize]
        } else {
          -1
        };

        // Pass the full remaining instruction stream (not just one word):
        // multi-word ops (LOADKX, GETIMPORT, FASTCALL2K, NEWCLASSMEMBER,
        // CMPPROTO, …) read their aux word via `code[1]`, which would be
        // out of bounds on a length-1 slice. C++ passes a bare pointer.
        self.dump_instruction(&insns_slice[i..], &mut result, target_label);

        let op = insn.luau_opcode();
        // 变步长推进：等价 cpp `i += getOpLength(op)`
        for _ in 1..get_op_length(op) as usize {
          insns.next();
        }
      }

      dumpinstoffs[insns_slice.len()] = result.len() as i32;
    }

    (result, dumpinstoffs)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_everything.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_everything(&self) -> String {
    let mut result = String::new();

    for (i, function) in self.functions.iter().enumerate() {
      let debugname = if function.dumpname.is_empty() {
        "??"
      } else {
        &function.dumpname
      };

      format_append(
        &mut result,
        format_args!("Function {} ({}):\n", i as i32, debugname),
      );

      result.push_str(&function.dump);
      result.push('\n');
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_function(&self, id: u32) -> String {
    LUAU_ASSERT!(id < self.functions.len() as u32);
    self.functions[id as usize].dump.clone()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_instruction.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn dump_instruction(
    &self,
    code: &[Instruction],
    result: &mut String,
    target_label: i32,
  ) -> usize {
    let insn = code[0];
    let op_enum = insn.luau_opcode();

    // 寄存器与操作数预取：const fn 位截取，无副作用，绑定后供下方各指令臂复用
    let a = insn.a() as u32;
    let b = insn.b() as u32;
    let c = insn.c() as u32;
    let d = insn.d() as i32;

    // K 常量内联样板：前缀格式串以 " [" 结尾，随后 dump_constant 输出常量体、"]\n" 收尾，块值为指令宽度。
    macro_rules! k_inline {
      ($fmt:literal, $($arg:expr),+ $(,)? ; $k:expr, $width:expr $(,)?) => {{
        format_append(result, format_args!($fmt, $($arg),+));
        self.dump_constant(result, $k, false);
        result.push_str("]\n");
        $width
      }};
    }

    // 单寄存器样板：<OP> R{a}
    macro_rules! r1 {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{}\n"), a));
        1
      }};
    }

    // 双寄存器样板：<OP> R{a} R{b}
    macro_rules! r2 {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} R{}\n"), a, b));
        1
      }};
    }

    // 三寄存器运算样板：<OP> R{a} R{b} R{c}
    macro_rules! r3 {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} R{}\n"), a, b, c),
        );
        1
      }};
    }

    // 二元 K 运算样板：<OP> R{a} R{b} K{c} [常量体]（ADDK/SUBK 等共用）
    macro_rules! rk_binary {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, c),
        );
        self.dump_constant(result, c as i32, false);
        result.push_str("]\n");
        1
      }};
    }

    // 表属性 KS 样板：<OP> R{a} R{b} K{aux} [常量体]（GETTABLEKS/SETTABLEKS/NAMECALL 共用）
    macro_rules! table_ks {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, code[1]),
        );
        self.dump_constant(result, code[1].raw() as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    // 表下标 TABLEN 样板：<OP> R{a} R{b} {c+1}
    macro_rules! table_n {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} {}\n"), a, b, c + 1),
        );
        1
      }};
    }

    // 上值操作样板：<OP> R{a} {b}
    macro_rules! r_upval {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} {}\n"), a, b));
        1
      }};
    }

    // 反向 K 运算样板：<OP> R{a} K{b} [常量体] R{c}（SUBRK/DIVRK 共用）
    macro_rules! rk_reverse {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} K{} ["), a, b));
        self.dump_constant(result, b as i32, false);
        format_append(result, format_args!("] R{}\n", c));
        1
      }};
    }

    // 比较跳转样板：<OP> R{a} R{b} L{target}，宽度 2
    macro_rules! jump_cmp {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} L{}\n"), a, code[1], target_label),
        );
        2
      }};
    }

    // 单寄存器条件跳转样板：<OP> R{a} L{target}，宽度 1
    macro_rules! jump_cond {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} L{}\n"), a, target_label),
        );
        1
      }};
    }

    // 纯 Label 跳转样板：<OP> L{target}，宽度 1
    macro_rules! jump_label {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " L{}\n"), target_label));
        1
      }};
    }

    // 条件跳转常量样板：<OP> R{a} K{aux_mask} L{target}{invert} [常量体]，宽度 2
    macro_rules! jump_xeq_k {
      ($op:literal) => {{
        let k = code[1].aux_kv();
        format_append(
          result,
          format_args!(
            concat!($op, " R{} K{} L{}{} ["),
            a,
            k,
            target_label,
            insn::invert_label(code[1].raw()),
          ),
        );
        self.dump_constant(result, k as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    // UDATA KS 族样板：<OP> R{a} R{b} K{aux16} [常量体]，宽度 2
    macro_rules! udata_ks {
      ($op:literal) => {{
        let k = code[1].aux_kv16() as u32;
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, k),
        );
        self.dump_constant(result, k as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    match op_enum {
      LuauOpcode::LOP_LOADNIL => r1!("LOADNIL"),
      LuauOpcode::LOP_LOADB => {
        if c != 0 {
          format_append(result, format_args!("LOADB R{a} {b} +{c}\n"));
        } else {
          format_append(result, format_args!("LOADB R{a} {b}\n"));
        }
        1
      }
      LuauOpcode::LOP_LOADN => {
        format_append(result, format_args!("LOADN R{a} {d}\n"));
        1
      }
      LuauOpcode::LOP_LOADK => k_inline!("LOADK R{} K{} [", a, d; d, 1),
      LuauOpcode::LOP_MOVE => r2!("MOVE"),
      LuauOpcode::LOP_GETGLOBAL => {
        k_inline!("GETGLOBAL R{} K{} [", a, code[1]; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_SETGLOBAL => {
        k_inline!("SETGLOBAL R{} K{} [", a, code[1]; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_GETUPVAL => r_upval!("GETUPVAL"),
      LuauOpcode::LOP_SETUPVAL => r_upval!("SETUPVAL"),
      LuauOpcode::LOP_CLOSEUPVALS => r1!("CLOSEUPVALS"),
      LuauOpcode::LOP_GETIMPORT => k_inline!("GETIMPORT R{} {} [", a, d; d, 2),
      LuauOpcode::LOP_GETTABLE => r3!("GETTABLE"),
      LuauOpcode::LOP_SETTABLE => r3!("SETTABLE"),
      LuauOpcode::LOP_GETTABLEKS => table_ks!("GETTABLEKS"),
      LuauOpcode::LOP_SETTABLEKS => table_ks!("SETTABLEKS"),
      LuauOpcode::LOP_GETTABLEN => table_n!("GETTABLEN"),
      LuauOpcode::LOP_SETTABLEN => table_n!("SETTABLEN"),
      LuauOpcode::LOP_NEWCLOSURE => {
        format_append(result, format_args!("NEWCLOSURE R{a} P{d}\n"));
        1
      }
      LuauOpcode::LOP_NAMECALL => table_ks!("NAMECALL"),
      LuauOpcode::LOP_CALL => {
        format_append(
          result,
          format_args!("CALL R{a} {} {}\n", b as i32 - 1, c as i32 - 1),
        );
        1
      }
      LuauOpcode::LOP_CALLFB => {
        format_append(
          result,
          format_args!(
            "CALLFB R{a} {} {} [{}]\n",
            b as i32 - 1,
            c as i32 - 1,
            code[1].raw() as i32
          ),
        );
        2
      }
      LuauOpcode::LOP_RETURN => {
        format_append(result, format_args!("RETURN R{a} {}\n", b as i32 - 1));
        1
      }
      LuauOpcode::LOP_JUMP => jump_label!("JUMP"),
      LuauOpcode::LOP_JUMPIF => jump_cond!("JUMPIF"),
      LuauOpcode::LOP_JUMPIFNOT => jump_cond!("JUMPIFNOT"),
      LuauOpcode::LOP_JUMPIFEQ => jump_cmp!("JUMPIFEQ"),
      LuauOpcode::LOP_JUMPIFLE => jump_cmp!("JUMPIFLE"),
      LuauOpcode::LOP_JUMPIFLT => jump_cmp!("JUMPIFLT"),
      LuauOpcode::LOP_JUMPIFNOTEQ => jump_cmp!("JUMPIFNOTEQ"),
      LuauOpcode::LOP_JUMPIFNOTLE => jump_cmp!("JUMPIFNOTLE"),
      LuauOpcode::LOP_JUMPIFNOTLT => jump_cmp!("JUMPIFNOTLT"),
      LuauOpcode::LOP_ADD => r3!("ADD"),
      LuauOpcode::LOP_SUB => r3!("SUB"),
      LuauOpcode::LOP_MUL => r3!("MUL"),
      LuauOpcode::LOP_DIV => r3!("DIV"),
      LuauOpcode::LOP_IDIV => r3!("IDIV"),
      LuauOpcode::LOP_MOD => r3!("MOD"),
      LuauOpcode::LOP_POW => r3!("POW"),
      LuauOpcode::LOP_ADDK => rk_binary!("ADDK"),
      LuauOpcode::LOP_SUBK => rk_binary!("SUBK"),
      LuauOpcode::LOP_MULK => rk_binary!("MULK"),
      LuauOpcode::LOP_DIVK => rk_binary!("DIVK"),
      LuauOpcode::LOP_IDIVK => rk_binary!("IDIVK"),
      LuauOpcode::LOP_MODK => rk_binary!("MODK"),
      LuauOpcode::LOP_POWK => rk_binary!("POWK"),
      LuauOpcode::LOP_SUBRK => rk_reverse!("SUBRK"),
      LuauOpcode::LOP_DIVRK => rk_reverse!("DIVRK"),
      LuauOpcode::LOP_AND => r3!("AND"),
      LuauOpcode::LOP_OR => r3!("OR"),
      LuauOpcode::LOP_ANDK => rk_binary!("ANDK"),
      LuauOpcode::LOP_ORK => rk_binary!("ORK"),
      LuauOpcode::LOP_CONCAT => r3!("CONCAT"),
      LuauOpcode::LOP_NOT => r2!("NOT"),
      LuauOpcode::LOP_MINUS => r2!("MINUS"),
      LuauOpcode::LOP_LENGTH => r2!("LENGTH"),
      LuauOpcode::LOP_NEWTABLE => {
        format_append(
          result,
          format_args!(
            "NEWTABLE R{a} {} {}\n",
            if b == 0 { 0 } else { 1 << (b as i32 - 1) },
            code[1]
          ),
        );
        2
      }
      LuauOpcode::LOP_DUPTABLE => {
        format_append(result, format_args!("DUPTABLE R{a} {d}\n"));
        1
      }
      LuauOpcode::LOP_SETLIST => {
        format_append(
          result,
          format_args!("SETLIST R{a} R{b} {} [{}]\n", c as i32 - 1, code[1]),
        );
        2
      }
      LuauOpcode::LOP_FORNPREP => jump_cond!("FORNPREP"),
      LuauOpcode::LOP_FORNLOOP => jump_cond!("FORNLOOP"),
      LuauOpcode::LOP_FORGPREP => jump_cond!("FORGPREP"),
      LuauOpcode::LOP_FORGLOOP => {
        format_append(
          result,
          format_args!(
            "FORGLOOP R{} L{} {}{}\n",
            a,
            target_label,
            code[1].aux_a(),
            if (code[1].raw() as i32) < 0 {
              " [inext]"
            } else {
              ""
            }
          ),
        );
        2
      }
      LuauOpcode::LOP_FORGPREP_INEXT => jump_cond!("FORGPREP_INEXT"),
      LuauOpcode::LOP_FORGPREP_NEXT => jump_cond!("FORGPREP_NEXT"),
      LuauOpcode::LOP_GETVARARGS => {
        format_append(result, format_args!("GETVARARGS R{a} {}\n", b as i32 - 1));
        1
      }
      LuauOpcode::LOP_DUPCLOSURE => k_inline!("DUPCLOSURE R{} K{} [", a, d; d, 1),
      LuauOpcode::LOP_BREAK => {
        result.push_str("BREAK\n");
        1
      }
      LuauOpcode::LOP_JUMPBACK => jump_label!("JUMPBACK"),
      LuauOpcode::LOP_LOADKX => k_inline!("LOADKX R{} K{} [", a, code[1]; code[1].raw() as i32, 2),
      LuauOpcode::LOP_JUMPX => jump_label!("JUMPX"),
      LuauOpcode::LOP_FASTCALL => {
        format_append(result, format_args!("FASTCALL {a} L{target_label}\n"));
        1
      }
      LuauOpcode::LOP_FASTCALL1 => {
        format_append(result, format_args!("FASTCALL1 {a} R{b} L{target_label}\n"));
        1
      }
      LuauOpcode::LOP_FASTCALL2 => {
        format_append(
          result,
          format_args!("FASTCALL2 {a} R{b} R{} L{target_label}\n", code[1].aux_a()),
        );
        2
      }
      LuauOpcode::LOP_FASTCALL2K => {
        k_inline!(
          "FASTCALL2K {} R{} K{} L{} [",
          a,
          b,
          code[1],
          target_label;
          code[1].raw() as i32,
          2
        )
      }
      LuauOpcode::LOP_FASTCALL3 => {
        format_append(
          result,
          format_args!(
            "FASTCALL3 {} R{} R{} R{} L{}\n",
            a,
            b,
            code[1].aux_a(),
            code[1].aux_b(),
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_COVERAGE => {
        result.push_str("COVERAGE\n");
        1
      }
      LuauOpcode::LOP_CAPTURE => {
        let cap_type = u8::try_from(a).ok().and_then(LuauCaptureType::from_repr);
        let capture_name: &'static str = cap_type.map_or("", |c| c.into());
        format_append(
          result,
          format_args!(
            "CAPTURE {} {}{b}\n",
            capture_name,
            if cap_type == Some(LuauCaptureType::LctUpval) {
              'U'
            } else {
              'R'
            },
          ),
        );
        1
      }
      LuauOpcode::LOP_JUMPXEQKNIL => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKNIL R{a} L{}{}\n",
            target_label,
            insn::invert_label(code[1].raw())
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKB R{a} {} L{}{}\n",
            code[1].aux_kb(),
            target_label,
            insn::invert_label(code[1].raw())
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKN => jump_xeq_k!("JUMPXEQKN"),
      LuauOpcode::LOP_JUMPXEQKS => jump_xeq_k!("JUMPXEQKS"),
      LuauOpcode::LOP_GETUDATAKS => udata_ks!("GETUDATAKS"),
      LuauOpcode::LOP_SETUDATAKS => udata_ks!("SETUDATAKS"),
      LuauOpcode::LOP_NAMECALLUDATA => udata_ks!("NAMECALLUDATA"),
      LuauOpcode::LOP_NEWCLASSMEMBER => {
        k_inline!("NEWCLASSMEMBER R{} R{} [", a, c; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_CMPPROTO => {
        format_append(
          result,
          format_args!("CMPPROTO R{a} #{} L{target_label}\n", code[1]),
        );
        2
      }
      LuauOpcode::LOP_FASTPCALL => {
        format_append(
          result,
          format_args!(
            "FASTPCALL {} L{target_label}\n",
            if a == 0 { "pcall" } else { "xpcall" },
          ),
        );
        1
      }
      LuauOpcode::LOP_NEWCLASS => {
        if b == K_INVALID_REG {
          k_inline!("NEWCLASS R{} no_base K{} {} [", a, code[1], c; code[1].raw() as i32, 2)
        } else {
          k_inline!("NEWCLASS R{} R{} K{} {} [", a, b, code[1], c; code[1].raw() as i32, 2)
        }
      }
      _ => {
        LUAU_ASSERT!(false);
        1
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_source_remarks.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_source_remarks(&self) -> String {
    let mut result = String::new();

    let mut remarks: Vec<(i32, String)> = self.dump_remarks.clone();
    // C++ `std::sort(remarks)` orders by the WHOLE (line, message) pair, so within a
    // line remarks are ordered lexicographically by text ("builtin ..." before
    // "inlining ...") and the consecutive-duplicate skip can collapse repeated inline
    // remarks. Sorting by line only left them in insertion order.
    remarks.sort();
    let mut remarks = remarks.into_iter().peekable();

    for (i, line) in self.dump_source.iter().enumerate() {
      let line_no = (i + 1) as i32;

      let indent: usize = line
        .bytes()
        .take_while(|&b| b == b' ' || b == b'\t')
        .count();

      // 归并输出归属当前行的 remark
      while let Some((_, msg)) = remarks.next_if(|(no, _)| *no == line_no) {
        format_append(
          &mut result,
          format_args!("{:.*}-- remark: {}\n", indent, line, msg),
        );

        // 跳过重复 remark（内联/展开导致）：cpp `remarks[next] == remarks[next-1]`
        // 是 (line, message) 整对相等，行不同不得吞并
        while remarks
          .next_if(|(no, m)| *no == line_no && *m == msg)
          .is_some()
        {}
      }

      result.push_str(line);
      if i + 1 < self.dump_source.len() {
        result.push('\n');
      }
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_type_info(&self) -> String {
    let mut result = String::new();

    for (i, function) in self.functions.iter().enumerate() {
      let typeinfo = &function.typeinfo;
      if typeinfo.is_empty() {
        continue;
      }

      let encoded_type = typeinfo[0];

      LUAU_ASSERT!(encoded_type == LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8);

      format_append(&mut result, format_args!("{}: function(", i));

      LUAU_ASSERT!(typeinfo.len() >= 2);

      let numparams = typeinfo[1];

      LUAU_ASSERT!((1 + numparams as usize - 1) < typeinfo.len());

      for (j, &et) in typeinfo[2..2 + numparams as usize].iter().enumerate() {
        let optional = if (et & LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8) != 0 {
          "?"
        } else {
          ""
        };

        let base_type_str = get_base_type_string(et);
        format_append(&mut result, format_args!("{}{}", base_type_str, optional));

        if j + 1 != numparams as usize {
          format_append(&mut result, format_args!(", "));
        }
      }

      format_append(&mut result, format_args!(")\n"));
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_abc.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_abc(&mut self, op: LuauOpcode, a: u8, b: u8, c: u8) {
    let insn = (op as u32)
      | ((a as u32) << insn::A_SHIFT)
      | ((b as u32) << insn::B_SHIFT)
      | ((c as u32) << insn::C_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_ad.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_ad(&mut self, op: LuauOpcode, a: u8, d: i16) {
    let insn = op as u32 | ((a as u32) << insn::A_SHIFT) | ((d as u16 as u32) << insn::B_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_aux.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_aux(&mut self, aux: u32) {
    self.push_insn(aux);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_e.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn emit_e(&mut self, op: LuauOpcode, e: i32) {
    let insn = (op as u32) | ((e as u32) << insn::A_SHIFT);

    self.push_insn(insn);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_emit_label.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn emit_label(&self) -> usize {
    self.insns.len()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_end_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// `cost`：cpp `endFunction(..., uint64_t cost)` 的内联开销模型，LPF_INLINABLE
  /// 时由 `write_function` 序列化进字节码（Compiler.cpp:621 传入）。
  pub fn end_function(&mut self, maxstacksize: u8, numupvalues: u8, flags: u8, cost: u64) {
    LUAU_ASSERT!(self.current_function.is_some());
    // 协议违例（未 begin 就 end）在 cpp 里是 `functions[-1]` 越界；断言 release
    // 放行时按 id 0 走安全路径，不再复制 UB。
    let current_function = self.current_function.unwrap_or(0) as usize;
    let dump = self.dump_function_ptr.map(|dump_fn| dump_fn(self));
    {
      let func = &mut self.functions[current_function];
      func.maxstacksize = maxstacksize;
      func.numupvalues = numupvalues;

      if let Some((dump, dumpinstoffs)) = dump {
        func.dump = dump;
        func.dumpinstoffs = dumpinstoffs;
      }
    }

    // cpp `#ifdef LUAU_ASSERTENABLED validate(); #endif`：紧随 maxstacksize /
    // numupvalues 落位之后、`encoder.encode` 置换操作码之前（`validate` 内部按
    // `LUAU_ASSERTENABLED` 常量短路，release 零开销）。缺了这一步，非法图会被
    // 静默写进 data，错误一路延后到反序列化/VM 才暴露。
    self.validate();

    // 字段级不相交借用: self.encoder 与 self.insns 可同时可变, 无需 unsafe
    if let Some(encoder) = self.encoder.as_mut() {
      encoder.encode(&mut self.insns);
    }

    // 序列化容量提示落在真正收集字节的 `data` 上（原先误 reserve 在随后被整体
    // 替换的旧 func.data 上，热路径每次序列化都要多次扩容重分配）。
    let mut data = Vec::with_capacity(32 + self.insns.len() * 7);
    self.write_function(&mut data, current_function as u32, flags, cost);
    self.functions[current_function].data = data;

    self.current_function = None;
    self.total_instruction_count += self.insns.len();

    self.insns.clear();
    self.lines.clear();
    self.constants.clear();
    self.protos.clear();
    self.jumps.clear();
    self.fb_slots.clear();
    self.table_shapes.clear();

    self.debug_locals.clear();
    self.debug_upvals.clear();

    self.typed_locals.clear();
    self.typed_upvals.clear();

    self.constant_map.clear();
    self.table_shape_map.clear();
    self.proto_map.clear();

    self.debug_remarks.clear();
    self.debug_remark_buffer.clear();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_expand_jumps.rs` ──
/// PC 区间可重映射的局部记录（`DebugLocal` / `TypedLocal` 共有形状）。
trait PcRangeRemap {
  fn pc_range(&mut self) -> (&mut u32, &mut u32);
}

impl PcRangeRemap for DebugLocal {
  #[inline]
  fn pc_range(&mut self) -> (&mut u32, &mut u32) {
    (&mut self.startpc, &mut self.endpc)
  }
}

impl PcRangeRemap for TypedLocal {
  #[inline]
  fn pc_range(&mut self) -> (&mut u32, &mut u32) {
    (&mut self.startpc, &mut self.endpc)
  }
}

/// endpc 为右开区间：先映射 endpc-1 再 +1，起点直接映射。
fn remap_pc_ranges<L: PcRangeRemap>(locals: &mut [L], remap: &[u32]) {
  for local in locals {
    let (startpc, endpc) = local.pc_range();
    *endpc = if *startpc != *endpc {
      remap[(*endpc - 1) as usize] + 1
    } else {
      remap[*endpc as usize]
    };
    *startpc = remap[*startpc as usize];
  }
}

impl<'a> BytecodeBuilder<'a> {
  /// 返回值：cpp 出参 `hasLongJumpError` — 展开后仍超出 JUMPX 24 位射程，
  /// 调用方（Compiler / BytecodeGraph）须按 cpp 语义报错或返回空串。
  pub fn expand_jumps(&mut self) -> bool {
    if !self.has_long_jumps {
      return false;
    }

    // we have some jump instructions that couldn't be patched which means their offset didn't fit into 16 bits
    // our strategy for replacing instructions is as follows: instead of
    //   OP jumpoffset
    // we will synthesize a jump trampoline before our instruction (note that jump offsets are relative to next instruction):
    //   JUMP +1
    //   JUMPX jumpoffset
    //   OP -2
    // the idea is that during forward execution, we will jump over JUMPX into OP; if OP decides to jump, it will jump to JUMPX
    // JUMPX can carry a 24-bit jump offset

    // jump trampolines expand the code size, which can increase existing jump distances.
    // because of this, we may need to expand jumps that previously fit into 16-bit just fine.
    // the worst-case expansion is 3x, so to be conservative we will repatch all jumps that have an offset >= 32767/3
    const K_MAX_JUMP_DISTANCE_CONSERVATIVE: i32 = 32767 / 3;

    // we will need to process jumps in order
    self.jumps.sort_by_key(|lhs| lhs.source);

    // first, let's add jump thunks for every jump with a distance that's too big
    // we will create new instruction buffers, with remap table keeping track of the moves: remap[oldpc] = newpc
    let mut remap: Vec<u32> = vec![0; self.insns.len()];

    let mut newinsns: Vec<u32> = Vec::with_capacity(self.insns.len());
    let mut newlines: Vec<i32> = Vec::with_capacity(self.insns.len());

    LUAU_ASSERT!(self.insns.len() == self.lines.len());

    let mut current_jump: usize = 0;
    let mut pending_trampolines: usize = 0;

    let mut i: usize = 0;
    while let Some(&word) = self.insns.get(i) {
      let insn = Instruction(word);
      let op = insn.op();
      LUAU_ASSERT!(op < LuauOpcode::LOP__COUNT as u8);

      if current_jump < self.jumps.len() && self.jumps[current_jump].source == i as u32 {
        let offset =
          (self.jumps[current_jump].target as i32) - (self.jumps[current_jump].source as i32) - 1;

        if offset.abs() > K_MAX_JUMP_DISTANCE_CONSERVATIVE {
          // insert jump trampoline as described above; we keep JUMPX offset uninitialized in this pass
          newinsns.push(LuauOpcode::LOP_JUMP as u32 | (1 << insn::B_SHIFT));
          newinsns.push(LuauOpcode::LOP_JUMPX as u32);

          newlines.push(self.lines[i]);
          newlines.push(self.lines[i]);

          pending_trampolines += 1;
        }

        current_jump += 1;
      }

      let oplen = get_op_length(insn.luau_opcode()) as usize;

      // copy instruction and line info to the new stream
      // remap 逐字递增：第 k 字映射到新流中的自身位置（cpp 同为 push 前取 size）
      let base = newinsns.len() as u32;
      newinsns.extend_from_slice(&self.insns[i..i + oplen]);
      newlines.extend(repeat_n(self.lines[i], oplen));
      for (k, r) in remap[i..i + oplen].iter_mut().enumerate() {
        *r = base + k as u32;
      }

      i += oplen;
    }

    LUAU_ASSERT!(current_jump == self.jumps.len());
    LUAU_ASSERT!(pending_trampolines > 0);

    // now we need to recompute offsets for jump instructions - we could not do this in the first pass because the offsets are between *target*
    // instructions
    for jump in &mut self.jumps {
      let offset = (jump.target as i32) - (jump.source as i32) - 1;
      let newoffset =
        (remap[jump.target as usize] as i32) - (remap[jump.source as usize] as i32) - 1;

      // cpp BytecodeBuilder.cpp:1407-1410：trampoline 展开后仍放不进 JUMPX 的
      // 24 位偏移 — 静默截断会产生坏字节码，立即放弃并上报
      if fflag::LuauCompileExpandLimit.get() && (newoffset.abs() + 1) >= K_MAX_JUMP_DISTANCE {
        return true;
      }

      if offset.abs() > K_MAX_JUMP_DISTANCE_CONSERVATIVE {
        // fix up jump trampoline
        let trampoline_pos = remap[jump.source as usize] as usize - 1;
        let op_pos = trampoline_pos + 1;

        let (left, right) = newinsns.split_at_mut(op_pos);
        let insnt = &mut left[trampoline_pos];
        let insnj = &mut right[0];

        LUAU_ASSERT!(Instruction(*insnt).opcode() == Some(LuauOpcode::LOP_JUMPX));

        // patch JUMPX to JUMPX to target location; note that newoffset is the offset of the jump *relative to OP*, so we need to add 1 to make it
        // relative to JUMPX
        *insnt &= insn::OP_MASK;
        *insnt |= ((newoffset + 1) as u32) << insn::A_SHIFT;

        // patch OP to OP -2
        *insnj &= insn::AD_MASK;
        *insnj |= ((-2i16) as u32) << insn::B_SHIFT;

        pending_trampolines -= 1;
      } else {
        let jump_insn = &mut newinsns[remap[jump.source as usize] as usize];

        // make sure jump instruction had the correct offset before we started
        LUAU_ASSERT!(Instruction(*jump_insn).d() as i32 == offset);

        // patch instruction with the new offset
        LUAU_ASSERT!(i32::from(newoffset as i16) == newoffset);

        *jump_insn &= insn::AD_MASK;
        *jump_insn |= (newoffset as u32) << insn::B_SHIFT;
      }
    }

    LUAU_ASSERT!(pending_trampolines == 0);

    // this was hard, but we're done.
    mem::swap(&mut self.insns, &mut newinsns);
    mem::swap(&mut self.lines, &mut newlines);

    remap_pc_ranges(&mut self.debug_locals, &remap);
    remap_pc_ranges(&mut self.typed_locals, &remap);

    false
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_finalize.rs` ──
// Source: `Bytecode/src/BytecodeBuilder.cpp:676`
//
// Faithful port of `BytecodeBuilder::finalize`: assemble the final bytecode
// blob — version byte, type-encoding version, string table, userdata type-name
// mapping, then every function's pre-serialized `data` blob, then the main
// function index. `bytecode` is taken out of `self` while writing so the
// `&self` helpers (`write_string_table`) and `&mut self` field reads don't
// alias the Buffer being filled.

impl<'a> BytecodeBuilder<'a> {
  pub fn finalize(&mut self) {
    LUAU_ASSERT!(self.bytecode.is_empty());

    // 已使用的 userdata 类型名先注册进字符串表。`UserdataType::name` 是 `StringRef<'a>`
    // （借用上游名表的视图，clone 不拷贝字节），因此这里把 `self` 拆成互不相交的
    // 字段借用即可完成，无需索引循环或裸指针。
    {
      let BytecodeBuilder {
        userdata_types,
        string_table,
        debug_strings,
        dump_flags,
        ..
      } = self;
      for ty in userdata_types.iter_mut() {
        if ty.used {
          ty.name_ref = bytecode_builder_intern_string(
            string_table,
            debug_strings,
            *dump_flags,
            ty.name.clone(),
          );
        }
      }
    }

    // preallocate space for bytecode blob
    let mut capacity: usize = 16;

    for (string_ref, _index) in self.string_table.iter() {
      capacity += string_ref.len() + 2;
    }

    for func in &self.functions {
      capacity += func.data.len();
    }

    // assemble final bytecode blob — taken out of `self` so the Buffer is
    // not aliased by the `&self`/field reads below.
    let mut bytecode = take(&mut self.bytecode);
    bytecode.reserve(capacity);

    let version = self.get_version();
    // cpp 769: `(version >= LBC_VERSION_MIN && version <= LBC_VERSION_MAX) || version == LBC_VERSION_CLASSES`
    LUAU_ASSERT!(
      (version >= LuauBytecodeTag::LBC_VERSION_MIN.0 as u8
        && version <= LuauBytecodeTag::LBC_VERSION_MAX.0 as u8)
        || version == LuauBytecodeTag::LBC_VERSION_CLASSES.0 as u8
    );

    // 版本字节直接写入字节缓冲（Vec<u8>，无 UTF-8 不变量约束）。
    bytecode.push(version);

    let typesversion = self.get_type_encoding_version();
    LUAU_ASSERT!(
      typesversion >= LuauBytecodeTag::LBC_TYPE_VERSION_MIN.0 as u8
        && typesversion <= LuauBytecodeTag::LBC_TYPE_VERSION_MAX.0 as u8
    );
    write_byte(&mut bytecode, typesversion);

    self.write_string_table(&mut bytecode);

    // Write the mapping between used type name indices and their name
    for (i, userdata_type) in self.userdata_types.iter().enumerate() {
      if userdata_type.used {
        write_byte(&mut bytecode, (i + 1) as u8);
        write_var_int(&mut bytecode, userdata_type.name_ref as u64);
      }
    }

    // 0 marks the end of the mapping
    write_byte(&mut bytecode, 0);

    write_var_int(&mut bytecode, self.functions.len() as u64);

    // cpp 797-802：四个 flag 任一开启时，每个函数体前写 VarInt 长度前缀，
    // 供读取方在应用该 flag 的增量（cost model / double 常量 / fastcall /
    // class shape）前定位函数边界。谓词与 `write_function` 的 feedback/cost
    // 尾段共用单一源。
    let size_prefix = cost_section_enabled();

    for func in &self.functions {
      if size_prefix {
        write_var_int(&mut bytecode, func.data.len() as u64);
      }
      // func.data 的字节流原样写入字节缓冲。
      bytecode.extend_from_slice(&func.data);
    }

    LUAU_ASSERT!((self.main_function as usize) < self.functions.len());
    write_var_int(&mut bytecode, self.main_function as u64);

    self.bytecode = bytecode;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_fold_jumps.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn fold_jumps(&mut self) {
    // if our function has long jumps, some processing below can make jump instructions not-jumps (e.g. JUMP->RETURN)
    // it's safer to skip this processing
    if self.has_long_jumps {
      return;
    }

    for jump in &mut self.jumps {
      let jump_label: u32 = jump.source;
      let jump_insn = Instruction(self.insns[jump_label as usize]);

      // follow jump target through forward unconditional jumps
      // we only follow forward jumps to make sure the process terminates
      // NB: C++ computes this with SIGNED `int` — `LUAU_INSN_D` is the signed
      // jump offset (negative for backward jumps), so `jumpLabel + 1 + D`
      // must be signed arithmetic. The model used `u32` with `D as u32`,
      // which overflows on any backward jump (every loop's back-edge).
      let mut target_label: i32 = jump_label as i32 + 1 + jump_insn.d() as i32;
      LUAU_ASSERT!((target_label as usize) < self.insns.len());
      let mut target_insn = Instruction(self.insns[target_label as usize]);

      while target_insn.opcode() == Some(LuauOpcode::LOP_JUMP) && target_insn.d() >= 0 {
        target_label = target_label + 1 + target_insn.d() as i32;
        LUAU_ASSERT!((target_label as usize) < self.insns.len());
        target_insn = Instruction(self.insns[target_label as usize]);
      }

      let offset: i32 = target_label - jump_label as i32 - 1;

      // for unconditional jumps to RETURN, we can replace JUMP with RETURN
      if jump_insn.opcode() == Some(LuauOpcode::LOP_JUMP)
        && target_insn.opcode() == Some(LuauOpcode::LOP_RETURN)
      {
        self.insns[jump_label as usize] = target_insn.raw();
      } else if (offset as i16) as i32 == offset {
        let mut insn = self.insns[jump_label as usize];
        insn &= insn::AD_MASK;
        insn |= ((offset as u16) as u32) << insn::B_SHIFT;
        self.insns[jump_label as usize] = insn;
      }

      jump.target = target_label as u32;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_bytecode.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// finalize 后的字节码 blob 原始字节。
  pub fn get_bytecode(&self) -> &[u8] {
    LUAU_ASSERT!(!self.bytecode.is_empty()); // did you forget to call finalize?
    &self.bytecode
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_debug_pc.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_debug_pc(&self) -> u32 {
    self.insns.len() as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_error.rs` ──
/// 错误 blob 首字节标记：合法 blob 首字节为字节码版本（≥ 1），0 不可能是版本号，
/// 故 0 即「这不是字节码，是错误消息」（cpp BytecodeBuilder.cpp `getError` 同值）。
const ERROR_BLOB_MARKER: u8 = 0;

impl<'a> BytecodeBuilder<'a> {
  /// 错误字节码 blob：首字节为错误标记，其后为可读消息的原始字节。
  pub fn get_error(message: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(message.len() + 1);
    result.push(ERROR_BLOB_MARKER);
    result.extend_from_slice(message.as_bytes());

    result
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

// ── abs-r139：并自 `methods/bytecode_builder_get_import_id_bytecode_builder.rs` ──
/// import id 打包（与 GETIMPORT 的 aux 字同构）的单一算术源：高 2 位为组件数，
/// 各组件按 `import_component_shift` 布局从高到低填入 10 位槽。公开三目
/// `get_import_id`/`get_import_id2`/`get_import_id3` 对应 cpp `getImportId`
/// 的三个重载，只定元数；掩码校验（各组件按位或后仍须 ⊆ 10 位掩码）收口于此。
fn pack_import_id(ids: &[i32]) -> u32 {
  let or_all = ids.iter().fold(0u32, |acc, &id| acc | id as u32);
  LUAU_ASSERT!(or_all <= K_IMPORT_COMPONENT_MASK);

  let mut word = (ids.len() as u32) << K_IMPORT_COUNT_SHIFT;
  for (k, &id) in ids.iter().enumerate() {
    word |= (id as u32) << import_component_shift(k as u32);
  }
  word
}

impl<'a> BytecodeBuilder<'a> {
  /// 打包 import id（cpp `BytecodeBuilder::getImportId`）：解码侧
  /// `decompose_import_id` 共用同一组布局常量（import id 与 aux 同构打包）。
  pub fn get_import_id(id0: i32) -> u32 {
    pack_import_id(&[id0])
  }

  pub fn get_import_id2(id0: i32, id1: i32) -> u32 {
    pack_import_id(&[id0, id1])
  }

  pub fn get_import_id3(id0: i32, id1: i32, id2: i32) -> u32 {
    pack_import_id(&[id0, id1, id2])
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_instruction_count.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_instruction_count(&self) -> usize {
    self.insns.len()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_string_table.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// cpp `getStringTable()`（`Bytecode/src/BytecodeBuilder.cpp:3351`）：返回按 1 基索引
  /// 铺平的字符串表**原始字节**。Lua/Luau 字符串常量允许任意字节，这里不做任何
  /// UTF-8 判定，非法字节原样保留；需要显示时才由调用方 `from_utf8_lossy`。
  /// 未落位的槽（理论上不可达，`LUAU_ASSERT` 兜底）留 `&[]` 哨兵，
  /// 与 cpp `stringTable` 的空串语义一致，避免第二种非法串→空串改写路径扩散到调用方。
  pub fn get_string_table(&self) -> Vec<&[u8]> {
    let mut strings: Vec<&[u8]> = vec![b"".as_slice(); self.string_table.size()];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      if index > 0 && (index as usize) <= strings.len() {
        strings[index as usize - 1] = string_ref.as_bytes();
      }
    }
    strings
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_total_instruction_count.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn get_total_instruction_count(&self) -> usize {
    self.total_instruction_count
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_type_encoding_version.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn get_type_encoding_version(&self) -> u8 {
    // C++: `return LBC_TYPE_VERSION_TARGET;` (3). Was stubbed to 1, which made
    // the Rust compiler emit a v1 typeinfo Header the VM loader can't parse.
    LuauBytecodeTag::LBC_TYPE_VERSION_TARGET.0 as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_version.rs` ──
// Source: `Bytecode/src/BytecodeBuilder.cpp:1487-1503` (hand-ported).
//
// ```cpp
// uint8_t BytecodeBuilder::getVersion()
// {
//     if (FFlag::DebugLuauUserDefinedClasses)
//         return LBC_VERSION_CLASSES;
//     if (FFlag::LuauCompileFastpcall)
//         return 14;
//     if (FFlag::LuauCompileEmitVectorDouble)
//         return 13;
//     if (FFlag::LuauBytecodeCostModel)
//         return 12;
//     if (FFlag::LuauEmitCallFeedback)
//         return 11;
//     return LBC_VERSION_TARGET;
// }
// ```
//
// `LuauCompileFastpcall` / `LuauCompileEmitVectorDouble` / `LuauBytecodeCostModel`
// 三个分支保留（flag 定义见 `ulua-common/src/fflag.rs`，默认 false）——旧版
// 移植的 `UdataDirect→9 / IntegerType2→8 / DuptableConstantPack2→7` 逐 flag
// 链已被上游吸收进 `LBC_VERSION_TARGET = 9`，逐 flag bump 属过期语义。

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn get_version(&self) -> u8 {
    if fflag::DebugLuauUserDefinedClasses.get() {
      return LuauBytecodeTag::LBC_VERSION_CLASSES.0 as u8;
    }

    // 上游 cpp `getVersion` 直接回字面量；此处命名收口，版本↔格式增量的
    // 对应关系只在本块声明一次（改 bump 只动一处）。
    const VERSION_FASTPCALL: u8 = 14;
    const VERSION_VECTOR_DOUBLE: u8 = 13;
    const VERSION_COST_MODEL: u8 = 12;
    const VERSION_CALL_FEEDBACK: u8 = 11;

    if fflag::LuauCompileFastpcall.get() {
      return VERSION_FASTPCALL;
    }
    if fflag::LuauCompileEmitVectorDouble.get() {
      return VERSION_VECTOR_DOUBLE;
    }
    if fflag::LuauBytecodeCostModel.get() {
      return VERSION_COST_MODEL;
    }

    if fflag::LuauEmitCallFeedback.get() {
      return VERSION_CALL_FEEDBACK;
    }

    LuauBytecodeTag::LBC_VERSION_TARGET.0 as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_needs_debug_remarks.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn needs_debug_remarks(&self) -> bool {
    DumpFlags::Remarks.is_set(self.dump_flags)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_aux.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_aux(&mut self, target_aux: usize, new_value: i32) {
    LUAU_ASSERT!(target_aux < self.insns.len());
    self.insns[target_aux] = new_value as u32;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_jump_d.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_jump_d(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = Instruction(self.insns[jump_label]);

    LUAU_ASSERT!(is_jump_d(jump_insn.luau_opcode()));
    LUAU_ASSERT!(jump_insn.d() == 0);

    LUAU_ASSERT!(target_label <= self.insns.len());

    let offset = target_label as i32 - jump_label as i32 - 1;

    if (offset as i16) as i32 == offset {
      self.insns[jump_label] |= ((offset as u16) as u32) << insn::B_SHIFT;
    } else if offset.abs() < K_MAX_JUMP_DISTANCE {
      // 16 位放不下：改走 JUMPX 蹦床重排（见 expandJumps）。上限取 JUMPX 的
      // 24 位射程 `kMaxJumpDistance`（并非 32767），约 800 万条指令内都能兜住。
      self.has_long_jumps = true;
    } else {
      return false;
    }

    self.jumps.push(Jump {
      source: jump_label as u32,
      target: target_label as u32,
    });

    true
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_patch_skip_c.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn patch_skip_c(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = Instruction(self.insns[jump_label]);

    let op = jump_insn.luau_opcode();
    LUAU_ASSERT!(is_skip_c(op) || is_fast_call(op));
    LUAU_ASSERT!(jump_insn.c() == 0);

    let offset = (target_label as i32) - (jump_label as i32) - 1;

    if (offset as u8) as i32 != offset {
      return false;
    }

    self.insns[jump_label] |= (offset as u32) << insn::C_SHIFT;
    true
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_debug_local.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_debug_local(&mut self, name: StringRef<'a>, reg: u8, startpc: u32, endpc: u32) {
    let index = self.add_string_table_entry(name);

    let local = DebugLocal {
      name: index,
      reg,
      startpc,
      endpc,
    };

    self.debug_locals.push(local);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_debug_upval.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_debug_upval(&mut self, name: StringRef<'a>) {
    let index = self.add_string_table_entry(name);

    let upval = DebugUpval { name: index };

    self.debug_upvals.push(upval);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_insn.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 指令与行号同步落盘的唯一入口：`emit_abc` / `emit_ad` / `emit_e` / `emit_aux`
  /// 四个编码变体只负责拼字，落盘（含行信息对齐）收敛于此。
  pub(crate) fn push_insn(&mut self, insn: u32) {
    self.insns.push(insn);
    self.lines.push(self.debug_line);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_local_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_local_type_info(
    &mut self,
    r#type: LuauBytecodeType,
    reg: u8,
    startpc: u32,
    endpc: u32,
  ) {
    let local = TypedLocal {
      r#type,
      reg,
      startpc,
      endpc,
    };

    self.typed_locals.push(local);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_upval_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_upval_type_info(&mut self, r#type: LuauBytecodeType) {
    let upval = TypedUpval { r#type };
    self.typed_upvals.push(upval);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_function_line_defined.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_function_line_defined(&mut self, line: i32) {
    // cpp `functions[currentFunction]` 在 -1（未 begin）时是越界 UB；这里
    // None 直接跳过写入，断言兜住 debug 下的协议违例。
    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].debuglinedefined = line;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_function_name.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_function_name(&mut self, name: StringRef<'a>) {
    let index = self.add_string_table_entry(name.clone());

    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].debugname = index;

      // cpp `functions[currentFunction].dumpname = std::string(name.data, name.length)`：
      // dumpname 是函数调试名（如 'foo'），供 DUPCLOSURE 常量 dump 展示，不是字节码
      // dump 文本。移植曾误在此处调用 dump 函数指针，故仅在 dump 开启时填充。
      if self.dump_function_ptr.is_some() {
        self.functions[id as usize].dumpname =
          String::from_utf8_lossy(name.as_bytes()).into_owned();
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_line.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_line(&mut self, line: i32) {
    self.debug_line = line;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_dump_flags.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_dump_flags(&mut self, flags: u32) {
    self.dump_flags = flags;
    self.dump_function_ptr = Some(BytecodeBuilder::dump_current_function);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_dump_source.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_dump_source(&mut self, source: &str) {
    // cpp `while (pos != npos)` 手工切行即 `split('\n')`：按下一个 '\n' 切块、
    // 无 '\n' 时产出余量，且以 '\n' 结尾时额外产出一个尾部空行——
    // dumpSourceRemarks 依赖该空行输出收尾换行，不可丢。CRLF 的行尾 '\r' 逐行剥除。
    self.dump_source = source
      .split('\n')
      .map(|line| line.strip_suffix('\r').unwrap_or(line).to_owned())
      .collect();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_function_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_function_type_info(&mut self, value: Vec<u8>) {
    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].typeinfo = value;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_main_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_main_function(&mut self, fid: u32) {
    LUAU_ASSERT!(fid < self.functions.len() as u32);

    self.main_function = fid;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_try_get_userdata_type_name.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn try_get_userdata_type_name(&self, type_: LuauBytecodeType) -> Option<&str> {
    // C++ `unsigned((type & ~LBC_TYPE_OPTIONAL_BIT) - LBC_TYPE_TAGGED_USERDATA_BASE)`: the
    // subtraction is done in (signed) int and cast to unsigned, so a non-userdata type wraps
    // to a huge index that fails the bounds check. The u16 subtraction here underflow-panicked.
    let index = ((type_.0 & !(LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0)) as i32
      - LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32) as u32;

    // C++ 返回 `userdataTypes[index].name` 这个 NUL 结尾的 `const char*`；这里按长度给出
    // `&str`（历史上返回裸指针后被调用方用 `CStr::from_ptr` 读取，会越过未终止的字节读到
    // 相邻内存，多带一个杂散字节 —— compiler_debug_types 失败的根因）。
    self
      .userdata_types
      .get(index as usize)
      .and_then(|ty| ty.name.as_str().ok())
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_undo_emit.rs` ──
// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。

impl<'a> BytecodeBuilder<'a> {
  pub fn undo_emit(&mut self, op: LuauOpcode) {
    LUAU_ASSERT!(!self.insns.is_empty());
    LUAU_ASSERT!((self.insns[self.insns.len() - 1] & insn::OP_MASK) == op as u32);

    self.insns.pop();
    self.lines.pop();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_use_userdata_type.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn use_userdata_type(&mut self, index: u32) {
    LUAU_ASSERT!((index as usize) < self.userdata_types.len());
    self.userdata_types[index as usize].used = true;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate_captures.rs` ──
/// cpp `std::array<bool, 256>`：寄存器编号是 u8，捕获位图固定 256 槽。
const CAPTURED_REG_COUNT: usize = 256;

/// cpp `validateCaptures` 内的局部 `struct BytecodeBlock`。
struct Block {
  startpc: usize,
  /// cpp 用 `-1` 表示"尚未定界"，这里用 `Option<usize>` 取代魔法数。
  finishpc: Option<usize>,
  captured: [bool; CAPTURED_REG_COUNT],
  visited: bool,
  in_worklist: bool,
  predecessors: SmallVector<u32, 4>,
  successors: SmallVector<u32, 4>,
}

impl Block {
  fn new(startpc: usize) -> Self {
    Self {
      startpc,
      finishpc: None,
      captured: [false; CAPTURED_REG_COUNT],
      visited: false,
      in_worklist: false,
      predecessors: SmallVector::new(),
      successors: SmallVector::new(),
    }
  }
}

impl BytecodeBuilder<'_> {
  /// cpp `validateCaptures()`（`BytecodeBuilder.cpp:2133-2312`）：重建字节码基本块图，
  /// 沿控制流传播「仍被 `CAPTURE REF` 打开的寄存器」位图，断言任何可达的 `RETURN`
  /// 处所有捕获都已被 `CLOSEUPVALS` 关闭。
  ///
  /// `validate_instructions` 里的 `open_captures` 只做线性扫描，看不到
  /// 「一个分支捕获、另一个分支直接 return」这类控制流漏洞，故上游另建这份 CFG 数据流校验。
  pub(crate) fn validate_captures(&self) {
    let insns = self.instructions();
    // cpp 的 `int` 下标在此收敛为 usize；空指令流是退化输入（cpp 会越界读 insns[0]），
    // 编译器不会产出，早退避免 panic。
    if insns.is_empty() {
      return;
    }

    // 1) 标记跳转目标（与 validate_variadic 共用的第一遍，见 mark_jump_targets）。
    let jump_targets = mark_jump_targets(insns);

    // 2) 重建基本块：落在跳转目标上的指令另起一块；终结指令（跳转/RETURN）结束当前块。
    // 下方两处 `blocks.last_mut().unwrap()` 是 100% 安全的：`blocks` 以
    // `vec![Block::new(0)]` 起步、循环内只 push 不 pop，恒非空。
    let mut blocks: Vec<Block> = vec![Block::new(0)];
    let mut previ = 0usize;
    let mut pc = 0usize;
    while let Some(insn) = insns.get(pc).copied() {
      let op = insn.luau_opcode();
      let nexti = pc + get_op_length(op) as usize;

      if pc != 0 && jump_targets[pc] {
        blocks.last_mut().unwrap().finishpc = Some(previ);
        blocks.push(Block::new(pc));
      }

      let target = get_jump_target(insn.raw(), pc as u32);
      if (target >= 0 && !is_fast_call(op)) || op == LuauOpcode::LOP_RETURN {
        blocks.last_mut().unwrap().finishpc = Some(pc);

        // 没有显式跳转承接 fallthrough 时，为后继指令新开一块
        if nexti < insns.len() && !jump_targets[nexti] {
          blocks.push(Block::new(nexti));
        }
      }

      previ = pc;
      pc = nexti;
    }
    if let Some(last) = blocks.last_mut()
      && last.finishpc.is_none()
    {
      last.finishpc = Some(previ);
    }

    // 3) 指令 → 所属块
    let mut owner_block_idx = vec![u32::MAX; insns.len()];
    for (block_idx, block) in blocks.iter().enumerate() {
      let Some(finishpc) = block.finishpc else {
        continue;
      };
      let mut pc = block.startpc;
      while pc <= finishpc
        && let Some(insn) = insns.get(pc).copied()
      {
        owner_block_idx[pc] = block_idx as u32;
        pc += get_op_length(insn.luau_opcode()) as usize;
      }
    }

    // 4) 收集前驱/后继
    // block_idx 是块编号：它会被原样写进 `successors`/`predecessors` 边表（u32 数据），
    // 且循环内既要可变改 `blocks[block_idx]`、又要 `blocks.get_mut(target_idx)`，
    // 两个槽位的借用无法用一次 iter_mut 表达，故保留下标遍历。
    for block_idx in 0..blocks.len() {
      // cpp 把循环变量与 `insn/op` 声明在块循环体内、指令循环体外，好让块尾的
      // fallthrough 复用终结指令的取值（未定界的块因此读到初值 `LOP_NOP`）；此处同样外提。
      // 未定界的块不扫指令，但 fallthrough 边仍按 cpp 语义尝试连接。
      let mut pc = blocks[block_idx].startpc;
      let mut insn = Instruction(0);
      let mut op = LuauOpcode::LOP_NOP;
      let finishpc = blocks[block_idx].finishpc;

      if let Some(finishpc) = finishpc {
        while pc <= finishpc
          && let Some(next_insn) = insns.get(pc).copied()
        {
          insn = next_insn;
          op = insn.luau_opcode();

          let target = get_jump_target(insn.raw(), pc as u32);
          if target >= 0 && !is_fast_call(op) {
            // 非法跳转目标可能越界或落在未定界的字节上：cpp 靠 LUAU_ASSERT 兜底，
            // Rust 侧断言后跳过该边，不能让 panic 或越界下标改变校验结论。
            let target_idx = usize::try_from(target)
              .ok()
              .and_then(|t| owner_block_idx.get(t).copied())
              .unwrap_or_else(|| {
                LUAU_ASSERT!(false, "jump target must be owned by a block");
                u32::MAX
              });
            LUAU_ASSERT!(target_idx != u32::MAX);

            if target_idx != u32::MAX {
              blocks[block_idx].successors.push(target_idx);

              if let Some(pred_block) = blocks.get_mut(target_idx as usize) {
                pred_block.predecessors.push(block_idx as u32);
              }
            }
          }

          pc += get_op_length(op) as usize;
        }
      }

      // fallthrough 边也要接上（LOADB 的"跳过"分支除外）
      if is_fallthrough(op) && !(is_skip_c(op) && insn.c() != 0) && pc < insns.len() {
        let target_idx = owner_block_idx[pc];
        LUAU_ASSERT!(target_idx != u32::MAX && block_idx as u32 != target_idx);

        if target_idx != u32::MAX {
          blocks[block_idx].successors.push(target_idx);

          if let Some(pred_block) = blocks.get_mut(target_idx as usize) {
            pred_block.predecessors.push(block_idx as u32);
          }
        }
      }
    }

    // 5) 工作表数据流迭代：沿控制流广播被捕获的局部变量位图
    let mut worklist: Vec<u32> = vec![0];
    blocks[0].in_worklist = true;

    while let Some(idx) = worklist.pop() {
      let idx = idx as usize;
      let old_captured = blocks[idx].captured;
      let mut captured = old_captured;

      // 汇入所有已访问前驱的出口状态
      for &pred in &blocks[idx].predecessors {
        if blocks[pred as usize].visited {
          for (c, &p) in captured.iter_mut().zip(&blocks[pred as usize].captured) {
            *c |= p;
          }
        }
      }

      // cpp 中 `finishpc` 为 -1 时该扫描循环整体不执行，块仍被标注视并继续下发后继，
      // 故这里用 `if let` 取代 `continue`/伪默认值。
      if let Some(finishpc) = blocks[idx].finishpc {
        let mut pc = blocks[idx].startpc;
        while pc <= finishpc
          && let Some(insn) = insns.get(pc).copied()
        {
          let op = insn.luau_opcode();

          match op {
            LuauOpcode::LOP_CLOSEUPVALS => {
              captured[insn.a() as usize..].fill(false);
            }
            LuauOpcode::LOP_CAPTURE => {
              if insn.a() == LuauCaptureType::LCT_REF as u8 {
                captured[insn.b() as usize] = true;
              }
            }
            LuauOpcode::LOP_RETURN => {
              LUAU_ASSERT!(captured.iter().all(|&c| !c));
            }
            _ => {}
          }

          pc += get_op_length(op) as usize;
        }
      }

      let changed = captured != old_captured;
      blocks[idx].captured = captured;
      blocks[idx].visited = true;
      blocks[idx].in_worklist = false;

      // 后继：内容变化或未访问过则重新入队（先拷出索引，避免与 blocks 的可变借用重叠）
      let successors: SmallVector<u32, 4> = blocks[idx].successors.iter().copied().collect();
      for succ_idx in successors.iter().copied() {
        let succ = succ_idx as usize;
        if (!blocks[succ].visited || changed) && !blocks[succ].in_worklist {
          worklist.push(succ_idx);
          blocks[succ].in_worklist = true;
        }
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate_instructions.rs` ──
/// NAMECALL/NAMECALLUDATA 之后必须紧跟调用指令：常规形态是 CALL，
/// fastcall 重写后是 CALLFB（cpp `BytecodeBuilder.cpp:1979-1983/2004-2008`
/// 两处同为 `LOP_CALL || LOP_CALLFB` 判定，抽公共辅助避免再漂移）。
fn is_call_op(insn: Instruction) -> bool {
  matches!(
    insn.opcode(),
    Some(LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB)
  )
}

/// cpp `i += getOpLength(op); LUAU_ASSERT(i <= insns.size());`
/// （`BytecodeBuilder.cpp:1608-1610/2037-2039`）：按 op 长度步进后越过指令流末尾
/// 说明上一条指令声明的长度吞掉了尾部，走断言收口；不允许迭代器静默耗尽丢痕。
fn step_over_insn(insns: &[Instruction], i: usize, op: LuauOpcode) -> usize {
  let next = i + get_op_length(op) as usize;
  LUAU_ASSERT!(next <= insns.len());
  next
}

/// fastcall 族（FASTCALL/1/2/2K/3 与 FASTPCALL）臂尾共用检查：相对跳转目标
/// 处必须落在 CALL 上。cpp `BytecodeBuilder.cpp` 六处同型判定，收口单源。
fn assert_fastcall_target(insns: &[Instruction], i: usize, insn: Instruction) {
  LUAU_ASSERT!(insns[i + 1 + insn.c() as usize].opcode() == Some(LuauOpcode::LOP_CALL));
}

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn validate_instructions(&self) {
    LUAU_ASSERT!(self.current_function.is_some());
    // release 断言放行时按 id 0 校验，避免 cpp `functions[-1]` 的越界读。
    let current_function = self.current_function.unwrap_or(0) as usize;

    let func = &self.functions[current_function];
    let insns = self.instructions();

    // tag instruction offsets so that we can validate jumps
    let mut insnvalid = vec![0u8; insns.len()];

    let mut i = 0usize;
    while let Some(insn) = insns.get(i).copied() {
      let op = insn.luau_opcode();

      insnvalid[i] = 1;

      i = step_over_insn(insns, i, op);
    }

    // validate individual instructions
    let mut i = 0usize;
    while let Some(insn) = insns.get(i).copied() {
      let op = insn.luau_opcode();

      match op {
        LuauOpcode::LOP_LOADNIL => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_LOADB => {
          VREG!(insn.a(), func);
          let b_val = insn.b();
          LUAU_ASSERT!(b_val == 0 || b_val == 1);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
        }
        LuauOpcode::LOP_LOADN => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_LOADK => {
          VREG!(insn.a(), func);
          VCONSTANY!(insn.d() as usize, self.constants);
        }
        LuauOpcode::LOP_MOVE => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_SETGLOBAL => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUPVAL | LuauOpcode::LOP_SETUPVAL => {
          VREG!(insn.a(), func);
          VUPVAL!(insn.b(), func);
        }
        LuauOpcode::LOP_CLOSEUPVALS => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_GETIMPORT => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Import, self.constants);
          let id = insns[i + 1].raw();
          let (count, components) = decode_import_aux(id);
          LUAU_ASSERT!(count != 0); // import chain with length 1-3
          for &component in &components[..count.min(components.len() as u32) as usize] {
            VCONST!(component as usize, String, self.constants);
          }
        }
        LuauOpcode::LOP_GETTABLE | LuauOpcode::LOP_SETTABLE => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_SETTABLEKS => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETTABLEN | LuauOpcode::LOP_SETTABLEN => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_NEWCLOSURE => {
          VREG!(insn.a(), func);
          let proto_idx = insn.d() as usize;
          LUAU_ASSERT!(proto_idx < self.protos.len());
          let proto_val = self.protos[proto_idx];
          LUAU_ASSERT!(proto_val < self.functions.len() as u32);
          let numupvalues = self.functions[proto_val as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= insns.len());
          for cinsn in insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(cinsn.opcode() == Some(LuauOpcode::LOP_CAPTURE));
          }
        }
        LuauOpcode::LOP_NAMECALL => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
          LUAU_ASSERT!(is_call_op(insns[i + 2]));
        }
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = (insn.b() as i32) - 1;
          let nresults = (insn.c() as i32) - 1;
          VREG!(insn.a(), func);
          VREGRANGE!(insn.a().wrapping_add(1), nparams, func);
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_RETURN => {
          let nresults = (insn.b() as i32) - 1;
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_JUMP => {
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          VREG!(insn.a(), func);
          VREG!(insns[i + 1].aux_a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKNIL | LuauOpcode::LOP_JUMPXEQKB => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKN => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].aux_kv() as usize, Number, self.constants);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_JUMPXEQKS => {
          VREG!(insn.a(), func);
          VCONST!(insns[i + 1].aux_kv() as usize, String, self.constants);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_IDIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_IDIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insn.c() as usize, Number, self.constants);
        }
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          VREG!(insn.a(), func);
          VCONST!(insn.b() as usize, Number, self.constants);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONSTANY!(insn.c() as usize, self.constants);
        }
        LuauOpcode::LOP_CONCAT => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VREG!(insn.c(), func);
          LUAU_ASSERT!(insn.b() <= insn.c());
        }
        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
        }
        LuauOpcode::LOP_NEWTABLE => {
          VREG!(insn.a(), func);
        }
        LuauOpcode::LOP_DUPTABLE => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Table, self.constants);
        }
        LuauOpcode::LOP_SETLIST => {
          let count = (insn.c() as i32) - 1;
          VREG!(insn.a(), func);
          VREGRANGE!(insn.b(), count, func);
        }
        LuauOpcode::LOP_FORNPREP | LuauOpcode::LOP_FORNLOOP => {
          VREG!(insn.a().wrapping_add(2), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FORGPREP => {
          VREG!(insn.a().wrapping_add(3), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FORGLOOP => {
          VREG!(
            insn.a().wrapping_add(2).wrapping_add(insns[i + 1].aux_a()),
            func
          );
          VJUMP!(insn.d(), i, insns, insnvalid);
          LUAU_ASSERT!(insns[i + 1].aux_a() >= 1);
        }
        LuauOpcode::LOP_FORGPREP_INEXT | LuauOpcode::LOP_FORGPREP_NEXT => {
          VREG!(insn.a().wrapping_add(4), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_GETVARARGS => {
          let nresults = (insn.b() as i32) - 1;
          VREGRANGE!(insn.a(), nresults, func);
        }
        LuauOpcode::LOP_DUPCLOSURE => {
          VREG!(insn.a(), func);
          VCONST!(insn.d() as usize, Closure, self.constants);
          let proto = self.constants[insn.d() as usize].as_closure();
          LUAU_ASSERT!(proto < self.functions.len() as u32);
          let numupvalues = self.functions[proto as usize].numupvalues as u32;

          // cpp CODEGEN_ASSERT 同款：CAPTURE 序列必须完整在指令流内
          LUAU_ASSERT!(i + 1 + numupvalues as usize <= insns.len());
          for cinsn in insns[i + 1..].iter().take(numupvalues as usize) {
            LUAU_ASSERT!(cinsn.opcode() == Some(LuauOpcode::LOP_CAPTURE));
            let capture_type = cinsn.a();
            LUAU_ASSERT!(
              capture_type == LuauCaptureType::LCT_VAL as u8
                || capture_type == LuauCaptureType::LCT_UPVAL as u8
            );
          }
        }
        LuauOpcode::LOP_PREPVARARGS => {
          LUAU_ASSERT!(insn.a() as u32 == func.numparams as u32);
          LUAU_ASSERT!(func.isvararg);
        }
        LuauOpcode::LOP_BREAK => {}
        LuauOpcode::LOP_JUMPBACK => {
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_LOADKX => {
          VREG!(insn.a(), func);
          VCONSTANY!(insns[i + 1].raw() as usize, self.constants);
        }
        LuauOpcode::LOP_JUMPX => {
          VJUMP!(insn.e(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FASTCALL => {
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_FASTCALL1 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_FASTCALL2 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VREG!(insns[i + 1].aux_a(), func);
        }
        LuauOpcode::LOP_FASTCALL2K => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VCONSTANY!(insns[i + 1].raw() as usize, self.constants);
        }
        LuauOpcode::LOP_FASTCALL3 => {
          VREG!(insn.b(), func);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
          VREG!(insns[i + 1].aux_a(), func);
          VREG!(insns[i + 1].aux_b(), func);
        }
        LuauOpcode::LOP_COVERAGE => {}
        LuauOpcode::LOP_CAPTURE => {
          let cap = LuauCaptureType::from_repr(insn.a());
          match cap {
            Some(LuauCaptureType::LctVal) | Some(LuauCaptureType::LctRef) => {
              VREG!(insn.b(), func);
            }
            Some(LuauCaptureType::LctUpval) => VUPVAL!(insn.b(), func),
            None => LUAU_ASSERT!(false, "Unsupported capture type"),
          }
        }
        LuauOpcode::LOP_NEWCLASSMEMBER => {
          VREG!(insn.a(), func);
          LUAU_ASSERT!(insn.b() == 0);
          VREG!(insn.c(), func);
          VCONST!(insns[i + 1].raw() as usize, String, self.constants);
        }
        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].aux_kv16() as usize, String, self.constants);
        }
        LuauOpcode::LOP_NAMECALLUDATA => {
          VREG!(insn.a(), func);
          VREG!(insn.b(), func);
          VCONST!(insns[i + 1].aux_kv16() as usize, String, self.constants);
          // cpp `BytecodeBuilder.cpp:2008`：CALLFB（fastcall 重写）同样合法
          LUAU_ASSERT!(is_call_op(insns[i + 2]));
        }
        LuauOpcode::LOP_CMPPROTO => {
          VREG!(insn.a(), func);
          VJUMP!(insn.d(), i, insns, insnvalid);
        }
        LuauOpcode::LOP_FASTPCALL => {
          // cpp `BytecodeBuilder.cpp:2017`：A 是 nresults 上限位，只允许 0/1
          LUAU_ASSERT!(insn.a() <= 1);
          VJUMP!(insn.c() as i32, i, insns, insnvalid);
          assert_fastcall_target(insns, i, insn);
        }
        LuauOpcode::LOP_NEWCLASS => {
          LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
          VREG!(insn.a(), func);
          let super_reg = insn.b();
          LUAU_ASSERT!(
            super_reg as u32 == K_INVALID_REG || (super_reg as usize) < func.maxstacksize as usize
          );
          let flags = insn.c();
          LUAU_ASSERT!(flags == 0 || flags == 1);
          VCONST!(insns[i + 1].raw() as usize, ClassShape, self.constants);
        }
        _ => {
          LUAU_ASSERT!(false, "Unsupported opcode");
        }
      }

      i = step_over_insn(insns, i, op);
    }
    // 捕获闭合检查在 cpp 中由独立的 `validateCaptures`（CFG 分析）承担，
    // 已由 `BytecodeBuilder::validate` 统一调用，这里不再做线性近似。
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate_variadic.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 校验 MULTRET 序列：产出/消费变长序列的指令必须成对出现，
  /// 且序列内部（含消费指令）不得作为跳转目标。
  pub(crate) fn validate_variadic(&self) {
    let mut variadic_seq = false;
    let insns_slice = self.instructions();
    // 第一遍：标记全部跳转目标（与 validate_captures 共用的第一遍）
    let insn_targets = mark_jump_targets(insns_slice);

    // 第二遍：状态机校验 producer/consumer/neutral 的配对关系
    let mut insns = insns_slice.iter().copied().enumerate();
    while let Some((i, insn)) = insns.next() {
      let op = insn.luau_opcode();

      if variadic_seq {
        LUAU_ASSERT!(!insn_targets[i]);
      }

      if op == LuauOpcode::LOP_CALL || op == LuauOpcode::LOP_CALLFB {
        // 注意：CALL 可能结束一个变长序列并同时开始新的序列
        if insn.b() == 0 {
          // 消费指令结束变长序列
          LUAU_ASSERT!(variadic_seq);
          variadic_seq = false;
        } else {
          // CALL 非中性指令，序列内只能是消费指令
          LUAU_ASSERT!(!variadic_seq);
        }

        if insn.c() == 0 {
          // 产出指令开启变长序列
          LUAU_ASSERT!(!variadic_seq);
          variadic_seq = true;
        }
      } else if op == LuauOpcode::LOP_GETVARARGS && insn.b() == 0 {
        // 产出指令开启变长序列
        LUAU_ASSERT!(!variadic_seq);
        variadic_seq = true;
      } else if (op == LuauOpcode::LOP_RETURN && insn.b() == 0)
        || (op == LuauOpcode::LOP_SETLIST && insn.c() == 0)
      {
        // 消费指令结束变长序列
        LUAU_ASSERT!(variadic_seq);
        variadic_seq = false;
      } else if op == LuauOpcode::LOP_FASTCALL || op == LuauOpcode::LOP_FASTPCALL {
        let call_target = (i as i32 + insn.c() as i32 + 1) as usize;
        LUAU_ASSERT!(
          call_target < insns_slice.len()
            && insns_slice[call_target].opcode() == Some(LuauOpcode::LOP_CALL)
        );

        if insns_slice[call_target].b() == 0 {
          // 消费指令链接的 CALL 稍后自行结束序列，此处只校验状态
          LUAU_ASSERT!(variadic_seq);
        } else {
          LUAU_ASSERT!(!variadic_seq);
        }
      } else if op == LuauOpcode::LOP_CLOSEUPVALS
        || op == LuauOpcode::LOP_NAMECALL
        || op == LuauOpcode::LOP_NAMECALLUDATA
        || op == LuauOpcode::LOP_GETIMPORT
        || op == LuauOpcode::LOP_MOVE
        || op == LuauOpcode::LOP_GETUPVAL
        || op == LuauOpcode::LOP_GETGLOBAL
        || op == LuauOpcode::LOP_GETTABLEKS
        || op == LuauOpcode::LOP_COVERAGE
      {
        // 变长序列内的中性指令：不改 L->top
      } else {
        LUAU_ASSERT!(!variadic_seq);
      }

      // 变步长推进：等价 cpp `i += getOpLength(op)`，跳过当前指令的后续槽位
      for _ in 1..get_op_length(op) as usize {
        insns.next();
      }
    }

    LUAU_ASSERT!(!variadic_seq);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_validate.rs` ──
/// 变步长扫描指令流，标记每个**真实控制流转移**的落点槽位：跳转类指令的目标、
/// 且非 fastcall 族（其「目标」是指向配对 CALL 的相对偏移，不是控制流边）。
/// `validate_captures`（块重建）与 `validate_variadic`（序列目标检查）共用的
/// 第一遍。越界目标静默跳过——非法字节码可能算出越界 target，cpp 直接写下标
/// 是 UB，这里以受检写入收口（合法构建产物不会出现）。
pub(crate) fn mark_jump_targets(insns: &[Instruction]) -> Vec<bool> {
  let mut targets = vec![false; insns.len()];
  let mut pc = 0;
  while let Some(insn) = insns.get(pc).copied() {
    let op = insn.luau_opcode();
    let target = get_jump_target(insn.raw(), pc as u32);
    if target >= 0
      && !is_fast_call(op)
      && let Some(slot) = targets.get_mut(target as usize)
    {
      *slot = true;
    }
    pc += get_op_length(op) as usize;
  }
  targets
}

impl<'a> BytecodeBuilder<'a> {
  /// cpp `BytecodeBuilder::validate()`：整段定义都在 `#ifdef LUAU_ASSERTENABLED`
  /// 之内，release 下不参与编译。这里用同一个运行时开关提前返回，`LUAU_ASSERTENABLED`
  /// 是 `const bool`，release 下整个函数体（含 `validate_instructions` 里 400 余行
  /// 校验）会被判为死分支消除，零开销。
  ///
  /// 注意：必须在 `encoder.encode` 置换操作码**之前**调用，否则 `LUAU_INSN_OP/D`
  /// 解出的全是置换后的值。
  pub(crate) fn validate(&self) {
    if !LUAU_ASSERTENABLED {
      return;
    }

    self.validate_instructions();
    self.validate_variadic();
    // cpp `validate()` 第三项：`validateCaptures`（CFG 版捕获闭合检查，
    // 取代旧版 `validateInstructions` 尾部的线性 open-captures 近似）。
    self.validate_captures();
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_class_shape.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_class_shape(&self, ss: &mut Vec<u8>, cs: &ClassShape) {
    write_var_int(ss, cs.class_name as u64);
    write_var_int(ss, cs.property_names.len() as u64);
    write_var_int(ss, cs.method_names.len() as u64);

    for &prop_name in &cs.property_names {
      write_var_int(ss, prop_name as u64);
    }

    for &method_name in &cs.method_names {
      write_var_int(ss, method_name as u64);
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// `cost`：cpp `writeFunction(ss, id, flags, cost)` 的内联开销模型，
  /// LPF_INLINABLE 时以 VarInt 写入（BytecodeBuilder.cpp:1041-1046）。
  pub(crate) fn write_function(&mut self, ss: &mut Vec<u8>, id: u32, flags: u8, cost: u64) {
    LUAU_ASSERT!(id < self.functions.len() as u32);
    let func = &self.functions[id as usize];

    // Header
    write_byte(ss, func.maxstacksize);
    write_byte(ss, func.numparams);
    write_byte(ss, func.numupvalues);
    write_byte(ss, if func.isvararg { 1 } else { 0 });

    write_byte(ss, flags);

    if !func.typeinfo.is_empty() || !self.typed_upvals.is_empty() || !self.typed_locals.is_empty() {
      // collect type info into a temporary string to know the overall size of type data
      self.temp_type_info.clear();
      write_var_int(&mut self.temp_type_info, func.typeinfo.len() as u64);
      write_var_int(&mut self.temp_type_info, self.typed_upvals.len() as u64);
      write_var_int(&mut self.temp_type_info, self.typed_locals.len() as u64);

      self.temp_type_info.extend_from_slice(&func.typeinfo);

      for l in &self.typed_upvals {
        write_byte(&mut self.temp_type_info, l.r#type.0 as u8);
      }

      for l in &self.typed_locals {
        write_byte(&mut self.temp_type_info, l.r#type.0 as u8);
        write_byte(&mut self.temp_type_info, l.reg);
        write_var_int(&mut self.temp_type_info, l.startpc as u64);
        LUAU_ASSERT!(l.endpc >= l.startpc);
        write_var_int(&mut self.temp_type_info, (l.endpc - l.startpc) as u64);
      }

      write_var_int(ss, self.temp_type_info.len() as u64);
      ss.extend_from_slice(&self.temp_type_info);
    } else {
      write_var_int(ss, 0);
    }

    // instructions
    write_var_int(ss, self.insns.len() as u64);

    for &insn in &self.insns {
      write_bytes(ss, &(insn as i32).to_ne_bytes());
    }

    // constants
    write_var_int(ss, self.constants.len() as u64);

    for c in &self.constants {
      match c {
        Constant::Nil => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8);
        }
        Constant::Boolean(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8);
          write_byte(ss, *value as u8);
        }
        Constant::Number(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8);
          write_bytes(ss, &value.to_ne_bytes());
        }
        Constant::Integer(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8);
          if *value < 0 {
            write_byte(ss, 1);
            // C++ `~(uint64_t)value + 1` 即负数的绝对值
            write_var_int(ss, value.unsigned_abs());
          } else {
            write_byte(ss, 0);
            write_var_int(ss, *value as u64);
          }
        }
        Constant::Vector(vec) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
          for &component in vec {
            write_bytes(ss, &component.to_ne_bytes());
          }
        }
        // cpp `BytecodeBuilder.cpp:899-916`：flag 关闭时降级为 4×float 的
        // LBC_CONSTANT_VECTOR，与旧版本字节码兼容。
        Constant::Vectord(vec) => {
          if fflag::LuauCompileEmitVectorDouble.get() {
            write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTORD.0 as u8);
            for &component in vec {
              write_bytes(ss, &component.to_ne_bytes());
            }
          } else {
            write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
            for &component in vec {
              write_bytes(ss, &(component as f32).to_ne_bytes());
            }
          }
        }
        Constant::String(index) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8);
          write_var_int(ss, *index as u64);
        }
        Constant::Import(iid) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8);
          write_bytes(ss, &(*iid as i32).to_ne_bytes());
        }
        Constant::Table(shape_idx) => {
          let shape = &self.table_shapes[*shape_idx as usize];
          // cpp BytecodeBuilder.cpp:967-980 仅看 has_constants（无 flag 门控）：
          // 两臂只差 tag 与「是否交错写 value」，坍缩为单写路径。
          let has_constants = shape.has_constants;
          write_byte(
            ss,
            if has_constants {
              LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8
            } else {
              LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8
            },
          );
          write_var_int(ss, shape.length as u64);
          // 定长数组按 length 切片后迭代（§3：免逐下标边界重查）；越界 `length`
          // 的 panic 边界与原 `[i]` 索引完全一致（切片 end 越界即 panic）。
          let n = shape.length as usize;
          for (&k, &c) in shape.keys[..n].iter().zip(&shape.constants[..n]) {
            write_var_int(ss, k as u64);
            if has_constants {
              write_bytes(ss, &c.to_ne_bytes());
            }
          }
        }
        Constant::Closure(fid) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8);
          write_var_int(ss, *fid as u64);
        }
        Constant::ClassShape(shape_idx) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0 as u8);
          let cs = &self.class_shapes[*shape_idx as usize];
          self.write_class_shape(ss, cs);
        }
      }
    }

    // child protos
    write_var_int(ss, self.protos.len() as u64);

    for &child in &self.protos {
      write_var_int(ss, child as u64);
    }

    // debug info
    write_var_int(ss, func.debuglinedefined as u64);
    write_var_int(ss, func.debugname as u64);

    let has_lines = self.lines.iter().all(|&line| line != 0);

    if has_lines {
      write_byte(ss, 1);

      self.write_line_info(ss);
    } else {
      write_byte(ss, 0);
    }

    let has_debug = !self.debug_locals.is_empty() || !self.debug_upvals.is_empty();

    if has_debug {
      write_byte(ss, 1);

      write_var_int(ss, self.debug_locals.len() as u64);

      for l in &self.debug_locals {
        write_var_int(ss, l.name as u64);
        write_var_int(ss, l.startpc as u64);
        write_var_int(ss, l.endpc as u64);
        write_byte(ss, l.reg);
      }

      write_var_int(ss, self.debug_upvals.len() as u64);

      for l in &self.debug_upvals {
        write_var_int(ss, l.name as u64);
      }
    } else {
      write_byte(ss, 0);
    }

    if fflag::LuauEmitCallFeedback.get() {
      // Feedback Slots
      write_var_int(ss, self.fb_slots.len() as u64);
      for &pc in &self.fb_slots {
        write_byte(ss, LuauFeedbackType::LFT_CALLTARGET as u8);
        write_var_int(ss, pc as u64);
      }
    } else if cost_section_enabled() {
      // cpp: `cpp/Bytecode/src/BytecodeBuilder.cpp:1036-1039` — Empty feedback vector
      write_var_int(ss, 0);
    }

    // cpp BytecodeBuilder.cpp:1041-1046 — LPF_INLINABLE 时写内联开销模型
    if cost_section_enabled() && (flags & LuauProtoFlag::LPF_INLINABLE as u8) != 0 {
      write_var_int(ss, cost);
    }
  }
}

/// feedback/cost 尾段随带的格式开关组合：cpp 同条件在 `writeFunction` 的
/// 1036 与 1041 两处、以及 `finalize` 的 797 处（函数体长度前缀）出现，
/// 此处收敛为单一谓词（任一开启即写入空 feedback 向量与 LPF_INLINABLE 的
/// cost VarInt，finalize 侧逐函数写长度前缀）。
pub(crate) fn cost_section_enabled() -> bool {
  fflag::LuauBytecodeCostModel.get()
    || fflag::LuauCompileEmitVectorDouble.get()
    || fflag::LuauCompileFastpcall.get()
    || fflag::DebugLuauUserDefinedClasses.get()
}

// ── abs-r139：并自 `methods/bytecode_builder_write_line_info.rs` ──
/// line-info 分组跨度初值（覆盖全表的足够大跨度）。
const INITIAL_SPAN: usize = 1 << 24;
/// 行号与组内基线之差的最大值（单字节增量编码）。
const MAX_LINE_DELTA: i32 = 255;

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_line_info(&self, ss: &mut Vec<u8>) {
    LUAU_ASSERT!(!self.lines.is_empty());

    let mut span = INITIAL_SPAN;

    // offset 是行号表的扫描位置（窗口推进量 = span，属数据语义），外层保留下标游走。
    // 内层定窗扫描改为切片顺序迭代：`advance` 为首个使 max-min 超出单字节增量的
    // 元素下标（与 cpp 在违例元素处 break、游标停在其位置一致），无违例则为窗口长。
    // min/max 只服务于此终止判定，编码期基线另由 chunks().min() 重算。
    let mut offset = 0;
    while offset < self.lines.len() {
      let window = &self.lines[offset..offset + span.min(self.lines.len() - offset)];
      let (mut min, mut max) = (window[0], window[0]);
      let mut advance = window.len();
      for (k, &line) in window.iter().enumerate() {
        min = min.min(line);
        max = max.max(line);
        if max - min > MAX_LINE_DELTA {
          advance = k;
          break;
        }
      }
      let next = offset + advance;

      if next < self.lines.len() && next - offset < span {
        span = 1 << log2((next - offset) as i32);
      }
      // C++ `for (...; offset += span)`：收缩后也按新 span 推进，
      // 与 calcLinesSpan 的扫描节奏一致，保证最终 span 与 C++ 输出逐字节一致
      offset += span;
    }

    let mut baseline_one = 0;
    let mut baseline_scratch = Vec::new();
    let baseline_size = (self.lines.len() - 1) / span + 1;

    if baseline_size > 1 {
      baseline_scratch.resize(baseline_size, 0);
    }

    let baseline = if baseline_size > 1 {
      &mut baseline_scratch
    } else {
      slice::from_mut(&mut baseline_one)
    };

    // 按 span 窗口分块求每窗口最小行号；chunks 保证每块非空
    for (window, chunk) in self.lines.chunks(span).enumerate() {
      baseline[window] = chunk.iter().copied().min().unwrap();
    }

    let logspan = log2(span as i32);
    write_byte(ss, logspan as u8);

    let mut last_offset = 0u8;
    for (i, &line) in self.lines.iter().enumerate() {
      let delta = line - baseline[i >> logspan];
      LUAU_ASSERT!((0..=MAX_LINE_DELTA).contains(&delta));

      write_byte(ss, (delta as u8).wrapping_sub(last_offset));
      last_offset = delta as u8;
    }

    let mut last_line = 0;
    for &line in &baseline[..baseline_size] {
      write_bytes(ss, &(line - last_line).to_ne_bytes());
      last_line = line;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_string_table.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_string_table(&self, ss: &mut Vec<u8>) {
    let count = self.string_table.size();
    // 槽位存共享引用而非 clone：序列化只读不改，省去每个条目一次堆拷贝；
    // 未落位槽按空串写出（与原 clone(default) 占位行为一致）。
    let empty = StringRef::default();
    let mut strings: Vec<Option<&StringRef<'a>>> = vec![None; count];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      strings[index as usize - 1] = Some(string_ref);
    }

    write_var_int(ss, strings.len() as u64);

    for slot in strings {
      let s = slot.unwrap_or(&empty);
      write_var_int(ss, s.len() as u64);
      // 字符串表条目为原始字节，Vec<u8> 直接追加，无 UTF-8 约束。
      ss.extend_from_slice(s.as_bytes());
    }
  }
}
