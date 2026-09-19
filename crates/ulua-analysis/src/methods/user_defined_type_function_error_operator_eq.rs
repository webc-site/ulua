use crate::records::user_defined_type_function_error::UserDefinedTypeFunctionError;

impl UserDefinedTypeFunctionError {
  #[inline]
  pub fn operator_eq(&self, rhs: &UserDefinedTypeFunctionError) -> bool {
    self.message == rhs.message
  }
}
