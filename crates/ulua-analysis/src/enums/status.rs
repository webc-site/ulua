use strum::IntoStaticStr;

/// 语句终止状态名即小写变体名（`unknown`/`continue`/`break`/`return`/`error`），
/// 与 cpp `getReason` 逐字一致，交 strum 派生（review.md §5 值↔枚举机械映射）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Status {
  Unknown,
  Continue,
  Break,
  Return,
  Error,
}
