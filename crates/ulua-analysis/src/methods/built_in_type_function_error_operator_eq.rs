use crate::records::{
  built_in_type_function_error::BuiltInTypeFunctionError, type_function_error::TypeFunctionError,
};

impl BuiltInTypeFunctionError {
  #[inline]
  pub fn operator_eq(&self, rhs: &BuiltInTypeFunctionError) -> bool {
    // TypeFunctionError doesn't implement PartialEq in this crate; compare its wrapped
    // fields via the existing operator_eq implementation (or fall back to pointer equality).
    // This avoids requiring PartialEq for TypeFunctionError.
    TypeFunctionError::operator_eq(&self.error, &rhs.error)
  }
}
