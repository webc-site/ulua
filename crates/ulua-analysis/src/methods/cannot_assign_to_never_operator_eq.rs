use crate::records::cannot_assign_to_never::CannotAssignToNever;

impl CannotAssignToNever {
  #[inline]
  pub fn operator_eq(&self, rhs: &CannotAssignToNever) -> bool {
    if self.cause.len() != rhs.cause.len() {
      return false;
    }

    for i in 0..self.cause.len() {
      if self.cause[i] != rhs.cause[i] {
        return false;
      }
    }

    self.rhs_type == rhs.rhs_type && self.reason == rhs.reason
  }
}
