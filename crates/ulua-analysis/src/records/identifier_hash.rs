#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IdentifierHash;

impl IdentifierHash {
  pub const fn new() -> Self {
    Self
  }
}
