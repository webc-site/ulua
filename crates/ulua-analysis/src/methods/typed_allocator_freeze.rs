use core::ffi::c_void;

use crate::{functions::paged_freeze::paged_freeze, records::typed_allocator::TypedAllocator};
impl<T> TypedAllocator<T> {
  pub fn freeze(&mut self) {
    for &block in &self.stuff {
      paged_freeze(block as *mut c_void, Self::K_BLOCK_SIZE_BYTES);
    }
    self.frozen = true;
  }
}
