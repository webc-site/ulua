use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  pub fn clear(&mut self) {
    if self.frozen {
      self.unfreeze();
    }
    self.free();

    self.current_block_size = Self::K_BLOCK_SIZE;
  }
}
