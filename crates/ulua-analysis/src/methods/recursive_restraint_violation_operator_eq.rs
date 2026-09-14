use crate::records::recursive_restraint_violation::RecursiveRestraintViolation;

impl RecursiveRestraintViolation {
  #[inline]
  pub fn operator_eq(&self, _rhs: &RecursiveRestraintViolation) -> bool {
    true
  }
}
