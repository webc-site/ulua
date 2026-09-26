use crate::records::type_function_error::TypeFunctionError;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltInTypeFunctionError {
  pub(crate) error: TypeFunctionError,
}
