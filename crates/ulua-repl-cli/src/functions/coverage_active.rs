use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of `bool coverage_active() { return gCoverage.l != nullptr; }`.
pub(crate) fn coverage_active() -> bool {
  // 可空性已由 `Coverage::l: Option<NonNull<_>>` 表达，is_some 即 cpp 的判空
  G_COVERAGE.with(|coverage| coverage.borrow().l.is_some())
}
