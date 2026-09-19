use crate::records::generic_error::GenericError;

impl GenericError {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericError) -> bool {
    self.message == rhs.message
  }
}
