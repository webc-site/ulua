//! Source: `tests/CodeAllocator.test.cpp`

use alloc::vec::Vec;
#[derive(Debug, Clone, Default)]
pub struct Info {
  pub unwind: Vec<u8>,
  /// create 回调收到的 block 首地址（以整数保存，供断言做地址算术，避免裸指针外泄）。
  pub block: usize,
  pub destroy_called: bool,
}
