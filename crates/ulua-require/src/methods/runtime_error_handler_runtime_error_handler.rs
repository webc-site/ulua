use alloc::string::String;

use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
  pub fn new(required_path: &str) -> Self {
    Self {
      error_prefix: format!("error requiring module \"{required_path}\": "),
      error_message: String::new(),
    }
  }
}
