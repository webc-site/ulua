#[repr(transparent)] // single-field: layout-compatible with Id (get_if relies on this)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bound<Id> {
  pub bound_to: Id,
}
