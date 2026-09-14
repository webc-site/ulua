use core::ptr::null_mut;

use crate::records::temp_buffer::TempBuffer;

impl<T> Default for TempBuffer<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> TempBuffer<T> {
  pub fn new() -> Self {
    Self {
      l: null_mut(),
      data: null_mut(),
      count: 0,
    }
  }
}
