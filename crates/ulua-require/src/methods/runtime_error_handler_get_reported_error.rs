use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
  /// 已报告错误的原始字节（cpp 返回 `const std::string&`）。
  pub fn get_reported_error(&self) -> &[u8] {
    &self.error_message
  }
}
