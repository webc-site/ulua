use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter,
  non_strict_function_definition_error::NonStrictFunctionDefinitionError,
};

impl ErrorConverter {
  pub fn operator_call_47(&self, e: &NonStrictFunctionDefinitionError) -> String {
    let mut result = String::new();
    if !e.function_name().is_empty() {
      result.push_str("in the function '");
      result.push_str(e.function_name());
      result.push_str("', '");
    }
    result.push_str("the argument '");
    result.push_str(e.argument());
    result.push_str("' is used in a way that will error at runtime");
    result
  }
}
