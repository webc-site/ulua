use alloc::string::{String, ToString};

use crate::records::{
  error_converter::ErrorConverter, multiple_nonviable_overloads::MultipleNonviableOverloads,
};

impl ErrorConverter {
  pub fn operator_call_46(&self, e: &MultipleNonviableOverloads) -> String {
    let mut result = String::from("None of the overloads for function that accept ");
    result.push_str(&e.attempted_arg_count.to_string());
    result.push_str(" arguments are compatible.");
    result
  }
}
