#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Unmapped {
  /// The index of the scope where the generic pack was quantified
  pub(crate) scope_index: usize,
}
