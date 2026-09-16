use alloc::string::String;

use crate::records::{
  checked_function_incorrect_args::CheckedFunctionIncorrectArgs, error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_6(&self, e: &CheckedFunctionIncorrectArgs) -> String {
    format!(
      "the function '{}' will error at runtime if it is not called with {} arguments, \
       but we are calling it here with {} arguments",
      e.function_name(),
      e.expected(),
      e.actual()
    )
  }
}
