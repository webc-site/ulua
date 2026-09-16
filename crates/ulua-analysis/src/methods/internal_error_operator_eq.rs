use crate::records::internal_error::InternalError;

impl InternalError {
  #[inline]
  pub fn operator_eq(&self, rhs: &InternalError) -> bool {
    self.message == rhs.message
  }
}
