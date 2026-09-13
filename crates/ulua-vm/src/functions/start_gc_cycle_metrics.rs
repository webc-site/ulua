#[cfg(feature = "luai_gcmetrics")]
#[cfg(feature = "luai_gcmetrics")]
use crate::functions::lua_clock::lua_clock;
#[cfg(feature = "luai_gcmetrics")]
use crate::records::global_state::global_State;

#[cfg(feature = "luai_gcmetrics")]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn start_gc_cycle_metrics(g: *mut global_State) {
  unsafe {
    (*g).gcmetrics.currcycle.starttimestamp = lua_clock();
    (*g).gcmetrics.currcycle.pausetime =
      (*g).gcmetrics.currcycle.starttimestamp - (*g).gcmetrics.lastcycle.endtimestamp;
  }
}
