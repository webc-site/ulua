use alloc::string::{String, ToString};

use crate::records::{
  checked_function_incorrect_args::CheckedFunctionIncorrectArgs, error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_6(&self, e: &CheckedFunctionIncorrectArgs) -> String {
    let mut result = String::from("the function '");
    result.push_str(e.function_name());
    result.push_str("' will error at runtime if it is not called with ");
    result.push_str(&e.expected().to_string());
    result.push_str(" arguments, but we are calling it here with ");
    result.push_str(&e.actual().to_string());
    result.push_str(" arguments");
    result
  }
}
