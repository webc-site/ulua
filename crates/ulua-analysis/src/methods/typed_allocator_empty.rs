use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
  pub fn empty(&self) -> bool {
    self.stuff.is_empty()
  }
}
