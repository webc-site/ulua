#[cfg(feature = "luai_gcmetrics")]
use ulua_common::clock_shim::monotonic_seconds;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::global_state::global_State;

#[cfg(feature = "luai_gcmetrics")]
/// # Safety
/// 调用方须保证：`g` 为存活 global_State 且 gcmetrics 字段可写；仅应在 GC 周期切换点调用——
/// 非起点调用会用错序的 lastcycle.endtimestamp 写出无意义 pausetime（纯统计污染，不致 UB 之外的
/// 结构破坏）。cpp lgc.cpp:216（startCycleMetrics 等价段）
pub(crate) unsafe fn start_gc_cycle_metrics(g: *mut global_State) {
  // Safety: 契约保证 `g` 存活且 gcmetrics 字段可写，块内仅写时间戳差值
  unsafe {
    (*g).gcmetrics.currcycle.starttimestamp = monotonic_seconds();
    (*g).gcmetrics.currcycle.pausetime =
      (*g).gcmetrics.currcycle.starttimestamp - (*g).gcmetrics.lastcycle.endtimestamp;
  }
}
