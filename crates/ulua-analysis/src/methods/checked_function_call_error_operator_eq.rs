use crate::records::checked_function_call_error::CheckedFunctionCallError;

impl CheckedFunctionCallError {
  #[inline]
  pub fn operator_eq(&self, rhs: &CheckedFunctionCallError) -> bool {
    self.expected == rhs.expected
      && self.passed == rhs.passed
      && self.checked_function_name == rhs.checked_function_name
      && self.argument_index == rhs.argument_index
  }
}
