use crate::records::type_function_missing::TypeFunctionMissing;

impl TypeFunctionMissing {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeFunctionMissing) -> bool {
    self.function_name == rhs.function_name
  }
}
