use core::convert::Infallible;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct ConstraintSolvingIncompleteError {
  pub(crate) _unused: Option<Infallible>,
}
