use crate::enums::variant::Variant;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index {
  /// The 0-based index to use for the lookup.
  pub(crate) index: usize,
  /// The sort of thing we're indexing from, this is used in stringifying the type path for errors.
  pub(crate) variant: Variant,
}
