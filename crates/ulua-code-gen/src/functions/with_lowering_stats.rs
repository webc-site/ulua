//! 可空 `LoweringStats` 裸指针的单点门面。
//!
//! code-gen 各统计上报点原本散落着「`!stats.is_null()` 判空 + `(*stats)` 双重解引用 +
//! 字段累加」三连样板（对应 cpp oracle 的 `if (stats) stats->...` 惯用法）。本门面把
//! 全部解引用收敛到唯一一处窄 unsafe，调用点退化为普通闭包。

use crate::records::lowering_stats::LoweringStats;

/// 对可空 `LoweringStats` 借用执行 `f`：`Some` 时临时借用重建可变视图并返回 `Some(R)`，
/// `None` 时不解引用、直接返回 `None`（闭包随之不执行，天然保留 cpp 侧 `if (stats)`
/// 的惰性统计语义，判空守卫逐处等价、累加时机不变）。
#[inline]
pub(crate) fn with_lowering_stats<R>(
  stats: Option<&mut LoweringStats>,
  f: impl FnOnce(&mut LoweringStats) -> R,
) -> Option<R> {
  let s = stats?;
  Some(f(s))
}
