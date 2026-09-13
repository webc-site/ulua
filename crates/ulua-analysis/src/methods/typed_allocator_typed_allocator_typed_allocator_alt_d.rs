use crate::records::typed_allocator::TypedAllocator;

impl<T> Drop for TypedAllocator<T> {
  fn drop(&mut self) {
    if self.frozen {
      self.unfreeze();
    }
    self.free();
  }
}
