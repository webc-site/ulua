use core::ptr::null_mut;

use crate::records::code_allocator::CodeAllocator;

impl CodeAllocator {
  pub fn code_allocator_usize_usize(&mut self, block_size: usize, max_total_size: usize) {
    self.code_allocator_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      None,
      null_mut(),
    );
  }
}
