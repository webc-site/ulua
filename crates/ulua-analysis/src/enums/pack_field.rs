#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackField {
  /// What arguments this type accepts.
  Arguments,
  /// What this type returns when called.
  Returns,
  /// The tail of a type pack.
  Tail,
}
