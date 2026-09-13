#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NativeStackGuard {
  pub(crate) high: usize,
  pub(crate) low: usize,
}
