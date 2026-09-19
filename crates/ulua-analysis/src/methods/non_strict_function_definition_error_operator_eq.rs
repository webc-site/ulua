use crate::records::non_strict_function_definition_error::NonStrictFunctionDefinitionError;

impl NonStrictFunctionDefinitionError {
  #[inline]
  pub fn operator_eq(&self, rhs: &NonStrictFunctionDefinitionError) -> bool {
    self.function_name == rhs.function_name
      && self.argument == rhs.argument
      && self.argument_type == rhs.argument_type
  }
}
