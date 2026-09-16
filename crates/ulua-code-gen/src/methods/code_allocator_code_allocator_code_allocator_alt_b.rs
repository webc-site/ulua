use core::{ffi::c_void, ptr::null_mut};

use crate::{
  records::code_allocator::CodeAllocator, type_aliases::allocation_callback::AllocationCallback,
};

impl CodeAllocator {
  pub fn code_allocator_usize_usize_allocation_callback_void(
    &mut self,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: Option<AllocationCallback>,
    allocation_callback_context: *mut c_void,
  ) {
    self.block_pos = null_mut();
    self.block_end = null_mut();
    self.blocks.clear();
    self.unwind_infos.clear();
    self.block_size = block_size;
    self.max_total_size = max_total_size;
    self.live_allocations = 0;
    self.allocation_callback = allocation_callback;
    self.allocation_callback_context = allocation_callback_context;
    self.destroyed = false;

    debug_assert!(block_size > CodeAllocator::K_MAX_RESERVED_DATA_SIZE);
    debug_assert!(max_total_size >= block_size);
  }
}
