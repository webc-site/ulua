#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
  Unknown,
  Continue,
  Break,
  Return,
  Error,
}
