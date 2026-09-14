use crate::records::generic_bounds_mismatch::GenericBoundsMismatch;

impl GenericBoundsMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericBoundsMismatch) -> bool {
    self.generic_name == rhs.generic_name
      && self.lower_bounds == rhs.lower_bounds
      && self.upper_bounds == rhs.upper_bounds
  }
}
