use crate::records::failed_to_compile::FailedToCompile;

impl FailedToCompile {
  #[inline]
  pub fn operator_eq(&self, rhs: &FailedToCompile) -> bool {
    self.function_name == rhs.function_name && self.compile_error == rhs.compile_error
  }
}
