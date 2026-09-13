use crate::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs;

impl CheckedFunctionIncorrectArgs {
  #[inline]
  pub fn operator_eq(&self, rhs: &CheckedFunctionIncorrectArgs) -> bool {
    self.function_name == rhs.function_name
      && self.expected == rhs.expected
      && self.actual == rhs.actual
  }
}
