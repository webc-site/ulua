use alloc::string::String;

use crate::{
  functions::{
    to_human_readable_index::to_human_readable_index, to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    checked_function_call_error::CheckedFunctionCallError, error_converter::ErrorConverter,
  },
};

impl ErrorConverter {
  pub fn operator_call_5(&self, e: &CheckedFunctionCallError) -> String {
    let mut result = String::from("the function '");
    result.push_str(e.checked_function_name());
    result.push_str("' expects to get a ");
    result.push_str(&to_string_type_id(e.expected()));
    result.push_str(" as its ");
    result.push_str(&to_human_readable_index(e.argument_index()));
    result.push_str(" argument, but is being given a ");
    result.push_str(&to_string_type_id(e.passed()));
    result
  }
}
