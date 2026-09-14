use crate::records::type_mismatch::TypeMismatch;

impl TypeMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeMismatch) -> bool {
    if self.error.is_some() != rhs.error.is_some() {
      return false;
    }

    if let (Some(lhs_err), Some(rhs_err)) = (&self.error, &rhs.error)
      && !lhs_err.operator_eq(rhs_err)
    {
      return false;
    }

    self.wanted_type == rhs.wanted_type
      && self.given_type == rhs.given_type
      && self.reason == rhs.reason
      && self.context == rhs.context
  }
}
