use crate::records::optional_value_access::OptionalValueAccess;

impl OptionalValueAccess {
  #[inline]
  pub fn operator_eq(&self, rhs: &OptionalValueAccess) -> bool {
    self.optional == rhs.optional
  }
}
