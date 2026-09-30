use strum::Display;

/// 块类别名即小写变体名（`entry`/`linear`/`condition`），与 cpp 转储标签逐字一致，
/// 交 strum 派生（review.md §5 值↔枚举机械映射）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[strum(serialize_all = "lowercase")]
#[repr(i32)]
pub enum BlockKind {
  Entry,
  Linear,
  Condition,
}
