//! 可空 `CompilationStats` 裸指针的单点门面。
//!
//! 编译期统计的上报点原本散落着「`!stats.is_null()` 判空 + `(*stats)` 双重解引用 +
//! 字段累加」三连样板（对应 cpp oracle 的 `if (stats) stats->...` 惯用法）。本门面把
//! 全部解引用收敛到唯一一处窄 unsafe，调用点退化为普通闭包。

use crate::records::compilation_stats::CompilationStats;

/// 对可空 `CompilationStats` 裸指针执行 `f`：非空时重建临时可变借用并返回 `Some(R)`，
/// 为空时不解引用、直接返回 `None`（闭包随之不执行，天然保留 cpp 侧的惰性统计语义）。
///
/// 不变量：`stats` 为 null，或在本调用点指向存活的 `CompilationStats`；闭包执行期间
/// 不存在对同一对象的其他可变借用（编译流程单线程串行，统计点即建即用）。
#[inline]
pub(crate) fn with_compilation_stats<R>(
  stats: *mut CompilationStats,
  f: impl FnOnce(&mut CompilationStats) -> R,
) -> Option<R> {
  if stats.is_null() {
    return None;
  }

  // Safety: 依上述不变量，`stats` 此刻指向存活且无并存借用的 CompilationStats；重建的
  // `&mut` 仅存活于闭包调用期间，随函数返回即结束，不与裸指针路径交叠。
  let s = unsafe { &mut *stats };
  Some(f(s))
}
