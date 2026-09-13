#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableState {
  Sealed,
  Unsealed,
  Free,
  Generic,
}
