use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  #[inline]
  pub fn is_frozen(&self) -> bool {
    self.frozen
  }
}
