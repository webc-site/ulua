use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  pub fn contains(&self, ptr: *const T) -> bool {
    for &block in &self.stuff {
      let block_ptr = block as *const T;
      let block_end = unsafe { block_ptr.add(Self::K_BLOCK_SIZE) };

      if ptr >= block_ptr && ptr < block_end {
        return true;
      }
    }

    false
  }
}
