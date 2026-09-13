#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AllocationData {
  pub bytes_allocated: usize,
  pub bytes_freed: usize,
}
