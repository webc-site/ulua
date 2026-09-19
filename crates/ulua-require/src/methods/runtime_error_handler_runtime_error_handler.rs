use alloc::vec::Vec;

use crate::records::runtime_error_handler::RuntimeErrorHandler;

/// cpp `errorPrefix` 的头尾字面量：`error requiring module "<path>": `。
const PREFIX_HEAD: &[u8] = b"error requiring module \"";
const PREFIX_TAIL: &[u8] = b"\": ";

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
}
