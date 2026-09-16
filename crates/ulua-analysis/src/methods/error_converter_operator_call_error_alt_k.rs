use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, function_requires_self::FunctionRequiresSelf,
};

impl ErrorConverter {
  pub fn operator_call_26(&self, _e: &FunctionRequiresSelf) -> String {
    String::from(
      "This function must be called with self. Did you mean to use a colon instead of a dot?",
    )
  }
}
