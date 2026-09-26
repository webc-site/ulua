#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum NavigateResult {
  Success,
  Ambiguous,
  NotFound,
}
