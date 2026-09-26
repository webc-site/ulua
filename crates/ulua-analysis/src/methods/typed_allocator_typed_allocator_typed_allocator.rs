use alloc::vec::Vec;

use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  pub fn new() -> Self {
    Self {
      frozen: false,
      stuff: Vec::new(),
      current_block_size: Self::K_BLOCK_SIZE,
      paged: false,
    }
  }
}

impl<T> Drop for TypedAllocator<T> {
  fn drop(&mut self) {
    if self.frozen {
      self.unfreeze();
    }
    self.free();
  }
}
