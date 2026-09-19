#[cfg(feature = "luai_gcmetrics")]
use crate::functions::lua_clock::lua_clock;
#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::records::global_state::global_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg(feature = "luai_gcmetrics")]
pub(crate) unsafe fn finish_gc_cycle_metrics(g: *mut global_State) {
  unsafe {
    (*g).gcmetrics.currcycle.endtimestamp = lua_clock();
    (*g).gcmetrics.currcycle.endtotalsizebytes = (*g).totalbytes;

    (*g).gcmetrics.completedcycles += 1;
    (*g).gcmetrics.lastcycle = (*g).gcmetrics.currcycle;
    (*g).gcmetrics.currcycle = GCCycleMetrics::default();

    (*g).gcmetrics.currcycle.starttotalsizebytes = (*g).totalbytes;
    (*g).gcmetrics.currcycle.heaptriggersizebytes = (*g).gc_threshold;
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg(not(feature = "luai_gcmetrics"))]
pub(crate) unsafe fn finish_gc_cycle_metrics(_g: *mut global_State) {}
