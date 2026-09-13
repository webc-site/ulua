use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of `bool coverage_active() { return gCoverage.l != nullptr; }`.
pub fn coverage_active() -> bool {
  unsafe { !(*core::ptr::addr_of!(G_COVERAGE)).l.is_null() }
}
