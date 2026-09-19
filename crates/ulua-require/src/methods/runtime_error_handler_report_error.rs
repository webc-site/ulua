use alloc::vec::Vec;

use crate::records::{error_handler::ErrorHandler, runtime_error_handler::RuntimeErrorHandler};

impl RuntimeErrorHandler {
  /// 报告错误：`errorPrefix + message`，按字节拼接（cpp 同）。
  pub fn report_error(&mut self, message: &[u8]) {
    // 复用已分配的缓冲，避免每次报告都重新分配
    self.error_message.clear();
    self
      .error_message
      .reserve(self.error_prefix.len() + message.len());
    self.error_message.extend_from_slice(&self.error_prefix);
    self.error_message.extend_from_slice(message);
  }
}

impl ErrorHandler for RuntimeErrorHandler {
  fn report_error(&mut self, message: Vec<u8>) {
    RuntimeErrorHandler::report_error(self, &message);
  }
}
