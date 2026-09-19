use alloc::vec::Vec;
use core::{ffi::c_void, ptr::null_mut};

use crate::type_aliases::allocation_callback::AllocationCallback;

#[derive(Debug)]
#[repr(C)]
pub struct CodeAllocator {
  pub context: *mut c_void,
  pub create_block_unwind_info: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      block: *mut u8,
      block_size: usize,
      start_offset: &mut usize,
    ) -> *mut c_void,
  >,
  pub destroy_block_unwind_info:
    Option<unsafe extern "C-unwind" fn(context: *mut c_void, unwind_data: *mut c_void)>,
  pub(crate) block_pos: *mut u8,
  pub(crate) block_end: *mut u8,
  pub(crate) blocks: Vec<*mut u8>,
  pub(crate) unwind_infos: Vec<*mut c_void>,
  pub(crate) block_size: usize,
  pub(crate) max_total_size: usize,
  pub(crate) live_allocations: usize,
  pub(crate) allocation_callback: Option<AllocationCallback>,
  pub(crate) allocation_callback_context: *mut c_void,
  pub(crate) destroyed: bool,
}

impl CodeAllocator {
  pub(crate) const K_MAX_RESERVED_DATA_SIZE: usize = 256;
}

impl Default for CodeAllocator {
  fn default() -> Self {
    Self {
      context: null_mut(),
      create_block_unwind_info: None,
      destroy_block_unwind_info: None,
      block_pos: null_mut(),
      block_end: null_mut(),
      blocks: Vec::new(),
      unwind_infos: Vec::new(),
      block_size: 0,
      max_total_size: 0,
      live_allocations: 0,
      allocation_callback: None,
      allocation_callback_context: null_mut(),
      destroyed: false,
    }
  }
}

impl Drop for CodeAllocator {
  fn drop(&mut self) {
    CodeAllocator::destroy(self);
  }
}
