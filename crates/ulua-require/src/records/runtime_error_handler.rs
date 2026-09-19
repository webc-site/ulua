use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};

use crate::functions::display::lossy;

/// 对应 cpp `RuntimeErrorHandler`：`errorPrefix`/`errorMessage` 在 cpp 中是
/// `std::string`，前缀内嵌 require 路径字节，故用 `Vec<u8>` 保字节语义。
#[derive(Clone)]
pub struct RuntimeErrorHandler {
  pub(crate) error_prefix: Vec<u8>,
  pub(crate) error_message: Vec<u8>,
}

/// Debug 输出走 lossy 文本视图（展示用途），字段本体仍是原始字节。
impl Debug for RuntimeErrorHandler {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("RuntimeErrorHandler")
      .field("error_prefix", &lossy(&self.error_prefix))
      .field("error_message", &lossy(&self.error_message))
      .finish()
  }
}
