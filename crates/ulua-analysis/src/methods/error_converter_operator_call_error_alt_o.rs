use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, syntax_error::SyntaxError};

impl ErrorConverter {
  pub fn operator_call_39(&self, e: &SyntaxError) -> String {
    String::from(e.message())
  }
}
