use crate::records::runtime_error::RuntimeError;

impl RuntimeError {
  #[inline]
  pub fn operator_eq(&self, rhs: &RuntimeError) -> bool {
    self.message == rhs.message
  }
}
