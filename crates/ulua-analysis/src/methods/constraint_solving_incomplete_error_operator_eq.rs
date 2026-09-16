use crate::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;

impl ConstraintSolvingIncompleteError {
  #[inline]
  pub fn operator_eq(&self, _rhs: &ConstraintSolvingIncompleteError) -> bool {
    true
  }
}
