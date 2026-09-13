#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationStatus {
  Success,
  Ambiguous,
  NotFound,
}
