use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  pub fn size(&self) -> usize {
    if self.stuff.is_empty() {
      0
    } else {
      Self::K_BLOCK_SIZE * (self.stuff.len() - 1) + self.current_block_size
    }
  }
}
