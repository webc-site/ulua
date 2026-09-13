#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TypeField {
  /// The table of a metatable type.
  Table,
  /// The metatable of a type. This could be a metatable type, a primitive
  /// type, a class type, or perhaps even a string singleton type.
  Metatable,
  /// The lower bound of this type, if one is present.
  LowerBound,
  /// The upper bound of this type, if present.
  UpperBound,
  /// The index type.
  IndexLookup,
  /// The indexer result type.
  IndexResult,
  /// The negated type, for negations.
  Negated,
  /// The variadic type for a type pack.
  Variadic,
}
