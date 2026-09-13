use alloc::string::String;

use crate::{
  functions::to_string_type_function_error::to_string_type_function_error,
  records::{
    built_in_type_function_error::BuiltInTypeFunctionError, error_converter::ErrorConverter,
  },
};

impl ErrorConverter {
  pub fn operator_call_2(&self, e: &BuiltInTypeFunctionError) -> String {
    to_string_type_function_error(&e.error)
  }
}
