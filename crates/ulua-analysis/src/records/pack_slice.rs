#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PackSlice {
  /// The 0-based index to start the slice at.
  pub(crate) start_index: usize,
}
