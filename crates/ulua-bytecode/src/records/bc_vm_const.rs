// （b）镜像定形：本件内 cpp 同形访问器/类型全仓零消费，为免降级触发 dead_code 升级而保持 pub；
// 非降级对象，勿删（批次账 b28-bc-rt-tail）。
use crate::{enums::bc_vm_const_kind::BcVmConstKind, macros::union_reader::UNION_READER};

/// cpp `BcVmConst` 的 `kind` 标签 + 匿名 union（`BytecodeGraph.h:152-166`）。
/// Rust 端用带载荷的 enum 合一建模（同 `ulua-compiler` 的 `Constant` 先例）：
/// 标签与数据不可能脱钩，读错分支的 UB 在类型层面不可表达，全部 unsafe 消除。
///
/// `String` 与上游 `std::string_view valueString` 一一对应：它**借用**调用方提供的
/// 字符串表（`fromFunctionBytecode(bytecode, strings)` 的 `strings`），既不拷贝也不泄漏，
/// 且保留原始字节（Lua 字符串常量允许非 UTF-8）。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BcVmConst<'a> {
  Nil,
  Boolean(bool),
  Number(f64),
  /// cpp `valueVectorf`
  Vector([f32; 4]),
  /// cpp `valueVectord`（`BytecodeGraph.h:160`）
  Vectord([f64; 4]),
  /// 借用字符串表的原始字节（`string_view` 语义，不要求 UTF-8）
  String(&'a [u8]),
  Import(u32),
  /// table shape 下标
  Table(u32),
  /// 子闭包 proto 下标
  Closure(u32),
  Integer(i64),
  /// cpp `valueClassShape`（`BytecodeGraph.h:166`）
  ClassShape(u32),
}

// 语义对齐 cpp `BcVmConst::operator==`：同变体再比载荷。浮点变体的 `Eq` 沿用
// 原手工声明（NaN 自反性与旧实现一致，作为 map 键时按位使用）。
impl Eq for BcVmConst<'_> {}

impl<'a> BcVmConst<'a> {
  /// 对应 cpp 默认构造（kind=Nil、成员清零）：作为「未填充」占位与 SCCP 的 nil 常量。
  pub fn new() -> Self {
    Self::Nil
  }

  /// 对应 cpp `BytecodeGraph.cpp:116` `fn.constants[i].valueString = readString(strings, ...)`：
  /// 直接借用字符串表里的原始字节，不做拷贝，也不要求 UTF-8。
  pub(crate) fn from_string(bytes: &'a [u8]) -> Self {
    Self::String(bytes)
  }

  /// 变体 → cpp `BcVmConstKind` 标签：仅供仍按标签做两两比较/分派的调用点
  /// （Sccp、图转 bytecode）使用，零成本。
  #[inline]
  pub const fn kind(&self) -> BcVmConstKind {
    match self {
      Self::Nil => BcVmConstKind::Nil,
      Self::Boolean(_) => BcVmConstKind::Boolean,
      Self::Number(_) => BcVmConstKind::Number,
      Self::Vector(_) => BcVmConstKind::Vector,
      Self::Vectord(_) => BcVmConstKind::Vectord,
      Self::String(_) => BcVmConstKind::String,
      Self::Import(_) => BcVmConstKind::Import,
      Self::Table(_) => BcVmConstKind::Table,
      Self::Closure(_) => BcVmConstKind::Closure,
      Self::Integer(_) => BcVmConstKind::Integer,
      Self::ClassShape(_) => BcVmConstKind::ClassShape,
    }
  }

  UNION_READER!("BcVmConst" {
    /// 联合体读取入口的安全化替身（`UNION_READER!` 生成）：调用方约定变体匹配
    /// （原 `debug_assert` 语义保留；release 下原实现读到的是任意位型，现在读到的
    /// 是一致零值，均属误用路径）。全仓散点读取一律经这些入口或 `match` 变体。
    pub(crate) fn as_boolean() -> bool = Boolean, false;
    /// 按双精度浮点读取活跃变体。
    pub fn as_number() -> f64 = Number, 0.0;
    /// 读活跃的字符串字节切片（`string_view` 语义：原始字节，不要求 UTF-8）。
    pub fn as_string() -> &'a [u8] = String, &[];
    /// 按 import id 读取活跃变体。
    pub fn as_import() -> u32 = Import, 0;
    /// 按子闭包 proto 下标读取活跃变体。
    pub fn as_closure() -> u32 = Closure, 0;
    /// 按 64 位有符号整型读取活跃变体。
    pub(crate) fn as_integer() -> i64 = Integer, 0;
  });
}

impl Default for BcVmConst<'_> {
  fn default() -> Self {
    Self::new()
  }
}
