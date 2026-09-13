use crate::records::type_function_error::TypeFunctionError;

impl TypeFunctionError {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeFunctionError) -> bool {
    self.location == rhs.location && self.module_name == rhs.module_name && self.data == rhs.data
  }
}
