use core::ptr::null_mut;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CodeAllocationData {
  pub start: *mut u8,
  pub size: usize,
  pub code_start: *mut u8,
  pub allocation_start: *mut u8,
  pub allocation_size: usize,
}

impl Default for CodeAllocationData {
  fn default() -> Self {
    Self {
      start: null_mut(),
      size: 0,
      code_start: null_mut(),
      allocation_start: null_mut(),
      allocation_size: 0,
    }
  }
}
