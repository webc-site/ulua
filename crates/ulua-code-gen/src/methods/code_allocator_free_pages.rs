use core::ptr::null_mut;

use crate::{
  functions::free_pages_impl_code_allocator::free_pages_impl,
  records::code_allocator::CodeAllocator,
};

impl CodeAllocator {
  pub fn free_pages(&self, mem: *mut u8, size: usize) {
    unsafe {
      let page_aligned_size = CodeAllocator::align_to_page_size(size);

      if let Some(callback) = self.allocation_callback {
        callback(
          self.allocation_callback_context,
          mem.cast(),
          page_aligned_size,
          null_mut(),
          0,
        );
      }

      free_pages_impl(mem, page_aligned_size);
    }
  }
}
