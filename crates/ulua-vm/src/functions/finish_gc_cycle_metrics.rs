#[cfg(feature = "luai_gcmetrics")]
use ulua_common::clock_shim::monotonic_seconds;

#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::records::global_state::global_State;

/// # Safety
/// `g` 必须指向存活的 `global_State`，且在 GC 一步结束后回到 GCSpause 时调用（`currcycle` 已由
/// start/step 填充）；本函数只归档计时字段，不触发分配、不解引用堆对象。违反则读写悬垂统计结构。
/// cpp 侧同处另写 starttotalsizebytes/heaptriggersizebytes 两字段，本端口无消费方，随字段裁除。
/// cpp lgc.cpp:220（finishGcCycleMetrics）。
//
// r15-v1 保留注记（w6d 逐点定性）：本体 7 处场域写读点（6 处 gcmetrics 归档
// 写 + 1 处 totalbytes 读）皆经函数参数域句柄进入，来源为调用方透传裸指针而
// 非状态开场字段的局部别名，属票面「函数参数来源」定性保留类，不入 gs_ref
// 门面迁移面；收口体形制同 nn_alias 判例，无散点可收。
#[cfg(feature = "luai_gcmetrics")]
pub(crate) unsafe fn finish_gc_cycle_metrics(g: *mut global_State) {
  // SAFETY: 契约保证 `g` 存活且 currcycle 已被 start/step 填充，块内归档至 lastcycle 不越出结构
  unsafe {
    (*g).gcmetrics.currcycle.endtimestamp = monotonic_seconds();
    (*g).gcmetrics.currcycle.endtotalsizebytes = (*g).totalbytes;

    (*g).gcmetrics.completedcycles += 1;
    (*g).gcmetrics.lastcycle = (*g).gcmetrics.currcycle;
    (*g).gcmetrics.currcycle = GCCycleMetrics::default();
  }
}

// 未启用 metrics 时为空实现，不解引用任何指针，降为 safe fn；唯一调用点位于 unsafe 块内，无需改动。
#[cfg(not(feature = "luai_gcmetrics"))]
pub(crate) fn finish_gc_cycle_metrics(_g: *mut global_State) {}
