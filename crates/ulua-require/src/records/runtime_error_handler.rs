use alloc::vec::Vec;

use crate::records::error_handler::ErrorHandler;

/// cpp `errorPrefix` 的头尾字面量：`error requiring module "<path>": `。
const PREFIX_HEAD: &[u8] = b"error requiring module \"";
const PREFIX_TAIL: &[u8] = b"\": ";

/// 对应 cpp `RuntimeErrorHandler`：`errorPrefix`/`errorMessage` 在 cpp 中是
/// `std::string`，前缀内嵌 require 路径字节，故用 `Vec<u8>` 保字节语义。
pub struct RuntimeErrorHandler {
  pub(crate) error_prefix: Vec<u8>,
  pub(crate) error_message: Vec<u8>,
}

impl RuntimeErrorHandler {
  /// `required_path` 为 require 路径原始字节，直接拼进前缀（cpp 同）。
  pub fn new(required_path: &[u8]) -> Self {
    let mut error_prefix =
      Vec::with_capacity(PREFIX_HEAD.len() + required_path.len() + PREFIX_TAIL.len());
    error_prefix.extend_from_slice(PREFIX_HEAD);
    error_prefix.extend_from_slice(required_path);
    error_prefix.extend_from_slice(PREFIX_TAIL);

    Self {
      error_prefix,
      error_message: Vec::new(),
    }
  }

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

  /// 已报告错误的原始字节（cpp 返回 `const std::string&`）。
  pub fn get_reported_error(&self) -> &[u8] {
    &self.error_message
  }
}

impl ErrorHandler for RuntimeErrorHandler {
  fn report_error(&mut self, message: Vec<u8>) {
    RuntimeErrorHandler::report_error(self, &message);
  }
}
