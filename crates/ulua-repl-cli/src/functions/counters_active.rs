use crate::functions::counters_init::G_COUNTERS;

// Faithful port of `bool counters_active() { return G_COUNTERS.l != nullptr; }`.
pub fn counters_active() -> bool {
  unsafe { !(*core::ptr::addr_of!(G_COUNTERS)).l.is_null() }
}
