use core::ffi::c_void;

use crate::{functions::paged_unfreeze::paged_unfreeze, records::typed_allocator::TypedAllocator};
impl<T> TypedAllocator<T> {
  pub fn unfreeze(&mut self) {
    for &block in &self.stuff {
      paged_unfreeze(block as *mut c_void, Self::K_BLOCK_SIZE_BYTES);
    }
    self.frozen = false;
  }
}
