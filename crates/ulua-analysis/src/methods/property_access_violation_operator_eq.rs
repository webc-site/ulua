use crate::records::property_access_violation::PropertyAccessViolation;

impl PropertyAccessViolation {
  #[inline]
  pub fn operator_eq(&self, rhs: &PropertyAccessViolation) -> bool {
    self.table == rhs.table && self.key == rhs.key && self.context == rhs.context
  }
}
