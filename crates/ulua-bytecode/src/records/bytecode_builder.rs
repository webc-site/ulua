use core::fmt::{self, Debug, Formatter};

use ulua_common::{
  records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseEqDefault},
  type_aliases::{dense_hash_default::DenseHashDefault, dense_hash_fast::DenseHashFast},
};

use crate::{
  enums::{dump_flags::DumpFlags, r#type::Type},
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::{
    bytecode_encoder::Encoder, class_shape::ClassShape, constant::Constant,
    constant_key::ConstantKey, debug_local_bytecode_builder::DebugLocal, debug_upval::DebugUpval,
    function::Function, jump::Jump, string_ref::StringRef, table_shape::TableShape,
    typed_local_bytecode_builder::TypedLocal, typed_upval::TypedUpval, userdata_type::UserdataType,
  },
};

// cpp 中 BytecodeBuilder 按值传递即浅拷贝（encoder 是裸指针）。Rust 端 encoder 已收敛为
// 内联 `Encoder`（无 Box<dyn>），故 Clone 可派生，语义与 cpp 拷贝构造一致。

/// `kMaxConstantCount = 1 << 23`（cpp BytecodeBuilder.h:20）。
pub(crate) const K_MAX_CONSTANT_COUNT: u32 = 1 << 23;
/// `kMaxClosureCount = 1 << 15`（cpp BytecodeBuilder.h:21）。
pub(crate) const K_MAX_CLOSURE_COUNT: u32 = 1 << 15;
/// `kMaxUpvalueCount = 200`（cpp BytecodeBuilder.h:14）。
pub(crate) const K_MAX_UPVALUE_COUNT: u32 = 200;
/// `kInvalidReg = 255`（cpp BytecodeBuilder.h:18）。
pub(crate) const K_INVALID_REG: u32 = 255;

impl Debug for BytecodeBuilder {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("BytecodeBuilder").finish_non_exhaustive()
  }
}

#[derive(Clone)]
pub struct BytecodeBuilder {
  pub(crate) functions: Vec<Function>,
  pub(crate) current_function: u32,
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

  pub(crate) userdata_types: Vec<UserdataType>,

  // 字符串表存在遍历依赖（`write_string_table`、`get_string_table`、`finalize`
  // 里的容量累加），故保持 FNV 版 `DenseHashDefault`，迭代序与改动前逐一相同。
  pub(crate) string_table:
    DenseHashMap<StringRef, u32, DenseHashDefault<StringRef>, DenseEqDefault<StringRef>>,
  pub(crate) debug_strings: Vec<StringRef>,

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

  pub(crate) dump_function_ptr: Option<fn(&BytecodeBuilder, &mut Vec<i32>) -> String>,
}

impl BytecodeBuilder {
  pub fn get_string_hash(key: StringRef) -> u32 {
    bytecode_builder_get_string_hash(key)
  }

  pub const DUMP_CODE: u32 = DumpFlags::Code as u32;
  pub const DUMP_LINES: u32 = DumpFlags::Lines as u32;
  pub const DUMP_SOURCE: u32 = DumpFlags::Source as u32;
  pub const DUMP_LOCALS: u32 = DumpFlags::Locals as u32;
  pub const DUMP_REMARKS: u32 = DumpFlags::Remarks as u32;
  pub const DUMP_TYPES: u32 = DumpFlags::Types as u32;
  pub const DUMP_CONSTANTS: u32 = DumpFlags::Constants as u32;
}

impl Default for BytecodeBuilder {
  fn default() -> Self {
    Self::new(None)
  }
}
