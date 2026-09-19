use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, multiple_nonviable_overloads::MultipleNonviableOverloads,
};

impl ErrorConverter {
  pub fn operator_call_46(&self, e: &MultipleNonviableOverloads) -> String {
    format!(
      "None of the overloads for function that accept {} arguments are compatible.",
      e.attempted_arg_count
    )
  }
}
