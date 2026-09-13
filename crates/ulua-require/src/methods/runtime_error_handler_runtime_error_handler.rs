use alloc::string::String;

use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
  pub fn new(required_path: String) -> Self {
    let mut error_prefix = String::from("error requiring module \"");
    error_prefix.push_str(&required_path);
    error_prefix.push_str("\": ");

    Self {
      error_prefix,
      error_message: String::new(),
    }
  }
}
