#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum SkipTestResult {
  /// If a type function is cyclic, it cannot be reduced, but maybe we can
  /// make a guess and offer a suggested annotation to the user.
  CyclicTypeFunction,

  /// Indicase that we will not be able to reduce this type function this
  /// time. Constraint resolution may cause this type function to become
  /// reducible later.
  Irreducible,

  /// A type function that cannot be reduced any further because it has no valid reduction.
  /// eg add<number, string>
  Stuck,

  /// Some type functions can operate on generic parameters
  Generic,

  /// We might be able to reduce this type function, but not yet.
  Defer,

  /// We can attempt to reduce this type function right now.
  #[default]
  Okay,
}
