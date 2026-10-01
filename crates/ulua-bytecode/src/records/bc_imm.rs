use crate::{enums::bc_imm_kind::BcImmKind, macros::union_reader::UNION_READER};

/// cpp `BcImm` 的 `BcImmKind` 标签 + 匿名 union。Rust 端用带载荷的 enum 合一建模
/// （同 `ulua-compiler` 的 `Constant` 先例）：类型标签与数据不再可能脱钩，
/// 读错分支的 UB 在类型层面不可表达，全部 unsafe 消除。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BcImm {
  Boolean(bool),
  Int(i32),
  Import(u32),
}

impl BcImm {
  /// 变体 → 原 `BcImmKind` 标签：仅供仍按 kind 做两两比较/分派的调用点
  /// （Sccp、图序列化）使用，本体零成本。
  #[inline]
  pub const fn kind(&self) -> BcImmKind {
    match self {
      Self::Boolean(_) => BcImmKind::Boolean,
      Self::Int(_) => BcImmKind::Int,
      Self::Import(_) => BcImmKind::Import,
    }
  }

  UNION_READER!("BcImm" {
    /// 联合体读取入口的安全化替身（`UNION_READER!` 生成）：调用方约定变体匹配
    /// （原 `debug_assert` 语义保留；release 下原实现读到的是任意位型，现在读到的
    /// 是一致零值，均属误用路径）。
    pub(crate) fn as_boolean() -> bool = Boolean, false;
    /// 按 32 位有符号整型读取活跃变体。
    pub fn as_int() -> i32 = Int, 0;
    /// 按 import id 读取活跃变体。
    pub(crate) fn as_import() -> u32 = Import, 0;
  });
}
