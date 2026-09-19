//! Source: `tests/CodeAllocator.test.cpp`

use alloc::vec::Vec;
use core::ptr::null_mut;
#[derive(Debug, Clone)]
pub struct Info {
  pub unwind: Vec<u8>,
  pub block: *mut u8,
  pub destroy_called: bool,
}

impl Default for Info {
  fn default() -> Self {
    Self {
      unwind: Vec::new(),
      block: null_mut(),
      destroy_called: false,
    }
  }
}
