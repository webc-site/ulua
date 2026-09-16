#[cfg(feature = "luai_gcmetrics")]
use crate::functions::lua_clock::lua_clock;
#[cfg(feature = "luai_gcmetrics")]
use crate::records::gc_cycle_metrics::GCCycleMetrics;
use crate::type_aliases::global_state::global_State;

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

#[cfg(not(feature = "luai_gcmetrics"))]
pub(crate) unsafe fn finish_gc_cycle_metrics(_g: *mut global_State) {}
