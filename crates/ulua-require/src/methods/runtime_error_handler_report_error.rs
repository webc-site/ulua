use alloc::string::String;

use crate::records::{error_handler::ErrorHandler, runtime_error_handler::RuntimeErrorHandler};

impl RuntimeErrorHandler {
  pub fn report_error(&mut self, message: String) {
    // 复用已分配的缓冲，避免每次报告都重新分配
    self.error_message.clear();
    self.error_message.push_str(&self.error_prefix);
    self.error_message.push_str(&message);
  }
}

impl ErrorHandler for RuntimeErrorHandler {
  fn report_error(&mut self, message: String) {
    RuntimeErrorHandler::report_error(self, message);
  }
}
