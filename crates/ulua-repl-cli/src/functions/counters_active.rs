use crate::functions::counters_init::G_COUNTERS;

// Faithful port of `bool counters_active() { return G_COUNTERS.l != nullptr; }`.
pub(crate) fn counters_active() -> bool {
  // 可空性已由 `Counters::l: Option<NonNull<_>>` 表达，is_some 即 cpp 的判空
  G_COUNTERS.with(|counters| counters.borrow().l.is_some())
}
