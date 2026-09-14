use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, illegal_require::IllegalRequire};

impl ErrorConverter {
  pub fn operator_call_28(&self, e: &IllegalRequire) -> String {
    let mut result = String::from("Cannot require module ");
    result.push_str(e.module_name());
    result.push_str(": ");
    result.push_str(e.reason());
    result
  }
}
