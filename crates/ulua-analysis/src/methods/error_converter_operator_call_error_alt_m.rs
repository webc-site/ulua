use alloc::string::String;

use crate::records::{error_converter::ErrorConverter, unknown_require::UnknownRequire};

impl ErrorConverter {
  pub fn operator_call_44(&self, e: &UnknownRequire) -> String {
    if e.module_path().is_empty() {
      String::from("Unknown require: unsupported path")
    } else {
      let mut result = String::from("Unknown require: ");
      result.push_str(e.module_path());
      result
    }
  }
}
