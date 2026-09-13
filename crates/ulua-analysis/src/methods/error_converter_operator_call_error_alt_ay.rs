use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, reserved_identifier::ReservedIdentifier};

impl ErrorConverter {
  pub fn operator_call_52(&self, e: &ReservedIdentifier) -> String {
    let mut result = String::from(e.name());
    result.push_str(" cannot be used as an identifier for a type function or alias");
    result
  }
}
