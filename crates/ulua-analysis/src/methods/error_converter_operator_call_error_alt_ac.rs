use alloc::string::String;

use crate::records::{
  duplicate_generic_parameter::DuplicateGenericParameter, error_converter::ErrorConverter,
};

impl ErrorConverter {
  pub fn operator_call_21(&self, e: &DuplicateGenericParameter) -> String {
    let mut result = String::from("Duplicate type parameter '");
    result.push_str(e.parameter_name());
    result.push('\'');
    result
  }
}
