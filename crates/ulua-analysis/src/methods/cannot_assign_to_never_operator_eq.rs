use crate::records::cannot_assign_to_never::CannotAssignToNever;

impl CannotAssignToNever {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotAssignToNever) -> bool {
    self.cause == rhs.cause
      && self.rhs_type == rhs.rhs_type
      && self.reason == rhs.reason
  }
}
