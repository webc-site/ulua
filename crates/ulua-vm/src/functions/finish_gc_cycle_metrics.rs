#[cfg(feature = "luai_gcmetrics")]
use ulua_common::clock_shim::monotonic_seconds;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::records::global_state::global_State;

/// # Safety
/// `g` 必须指向存活的 `global_State`，且在 GC 一步结束后回到 GCSpause 时调用（`currcycle` 已由
/// start/step 填充）；本函数只归档计时字段，不触发分配、不解引用堆对象。违反则读写悬垂统计结构。
/// cpp lgc.cpp:220（finishGcCycleMetrics）。
#[cfg(feature = "luai_gcmetrics")]
pub(crate) unsafe fn finish_gc_cycle_metrics(g: *mut global_State) {
  // Safety: 契约保证 `g` 存活且 currcycle 已被 start/step 填充，块内归档至 lastcycle 不越出结构
  unsafe {
    (*g).gcmetrics.currcycle.endtimestamp = monotonic_seconds();
    (*g).gcmetrics.currcycle.endtotalsizebytes = (*g).totalbytes;

    (*g).gcmetrics.completedcycles += 1;
    (*g).gcmetrics.lastcycle = (*g).gcmetrics.currcycle;
    (*g).gcmetrics.currcycle = GCCycleMetrics::default();

    (*g).gcmetrics.currcycle.starttotalsizebytes = (*g).totalbytes;
    (*g).gcmetrics.currcycle.heaptriggersizebytes = (*g).gc_threshold;
  }
}

// 未启用 metrics 时为空实现，不解引用任何指针，降为 safe fn；唯一调用点位于 unsafe 块内，无需改动。
#[cfg(not(feature = "luai_gcmetrics"))]
pub(crate) fn finish_gc_cycle_metrics(_g: *mut global_State) {}
