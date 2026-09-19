#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LineCounters {
  pub regular_executed: u64,
  pub fallback_executed: u64,
  pub vm_exit_taken: u64,
}
