#[cfg(feature = "luai_gcmetrics")]
use ulua_common::clock_shim::monotonic_seconds;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::global_state::global_State;

#[cfg(feature = "luai_gcmetrics")]
/// # Safety
/// 调用方须保证：`g` 为存活 global_State 且 gcmetrics 字段可写；仅应在 GC 周期切换点调用，
/// 否则 starttimestamp 与上一周期的计时区间错位（纯统计污染，不致结构破坏）。
/// cpp lgc.cpp:216（startCycleMetrics 等价段，cpp 的 pausetime 差值在本端口无消费方，
/// 随该字段一并裁除）。
pub(crate) unsafe fn start_gc_cycle_metrics(g: *mut global_State) {
  // SAFETY: 契约保证 `g` 存活且 gcmetrics 字段可写，块内仅写一个时间戳
  unsafe {
    (*g).gcmetrics.currcycle.starttimestamp = monotonic_seconds();
  }
}
