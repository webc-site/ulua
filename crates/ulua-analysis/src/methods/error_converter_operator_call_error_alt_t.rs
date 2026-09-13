use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, internal_error::InternalError};

impl ErrorConverter {
  pub fn operator_call_30(&self, e: &InternalError) -> String {
    String::from(e.message())
  }
}
