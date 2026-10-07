//! 可空 `CompilationStats` 裸指针的单点门面。
//!
//! 编译期统计的上报点原本散落着「`!stats.is_null()` 判空 + `(*stats)` 双重解引用 +
//! 字段累加」三连样板（对应 cpp oracle 的 `if (stats) stats->...` 惯用法）。本门面把
//! 全部解引用收敛到唯一一处窄 unsafe，调用点退化为普通闭包。

use crate::records::compilation_stats::CompilationStats;

/// 对可空 `CompilationStats` 借用执行 `f`：`Some` 时重建临时可变视图并返回 `Some(R)`，
/// `None` 时不解引用、直接返回 `None`（闭包随之不执行，天然保留 cpp 侧 `if (stats)`
/// 的惰性统计语义，判空守卫逐处等价、累加时机不变）。
#[inline]
pub(crate) fn with_compilation_stats<R>(
  stats: Option<&mut CompilationStats>,
  f: impl FnOnce(&mut CompilationStats) -> R,
) -> Option<R> {
  let s = stats?;
  Some(f(s))
}
