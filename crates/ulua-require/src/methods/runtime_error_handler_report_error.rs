use alloc::string::String;

use crate::records::{error_handler::ErrorHandler, runtime_error_handler::RuntimeErrorHandler};

impl RuntimeErrorHandler {
  pub fn report_error(&mut self, message: String) {
    self.error_message = self.error_prefix.clone() + &message;
  }
}

impl ErrorHandler for RuntimeErrorHandler {
  fn report_error(&mut self, message: String) {
    RuntimeErrorHandler::report_error(self, message);
  }
}
