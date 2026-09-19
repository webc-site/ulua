use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, generic_error::GenericError};

impl ErrorConverter {
  pub fn operator_call_27(&self, e: &GenericError) -> String {
    String::from(e.message())
  }
}
