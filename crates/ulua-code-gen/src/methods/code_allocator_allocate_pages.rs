use core::ptr::null_mut;

use crate::{
  functions::allocate_pages_impl_code_allocator::allocate_pages_impl,
  records::code_allocator::CodeAllocator,
};

impl CodeAllocator {
  pub fn allocate_pages(&self, size: usize) -> *mut u8 {
    unsafe {
      let page_aligned_size = CodeAllocator::align_to_page_size(size);

      let mem = allocate_pages_impl(page_aligned_size);
      if mem.is_null() {
        return null_mut();
      }

      if let Some(callback) = self.allocation_callback {
        callback(
          self.allocation_callback_context,
          null_mut(),
          0,
          mem.cast(),
          page_aligned_size,
        );
      }

      mem
    }
  }
}
