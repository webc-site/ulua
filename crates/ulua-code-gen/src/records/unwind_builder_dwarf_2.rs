use alloc::vec::Vec;
use core::{mem::zeroed, ptr::null_mut};

use crate::records::{
  unwind_builder::UnwindBuilder, unwind_function_dwarf_2::UnwindFunctionDwarf2,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderDwarf2 {
  pub base: UnwindBuilder,
  pub(crate) begin_offset: usize,
  pub(crate) unwind_functions: Vec<UnwindFunctionDwarf2>,
  pub(crate) raw_data: [u8; 1024],
  pub(crate) pos: *mut u8,
  pub(crate) fde_entry_start: *mut u8,
}

impl UnwindBuilderDwarf2 {
  /// `const int kCodeAlignFactor = 1;` (UnwindBuilderDwarf2.cpp:75)
  pub const K_CODE_ALIGN_FACTOR: i32 = 1;
  /// `const int kDataAlignFactor = 8;` (UnwindBuilderDwarf2.cpp:76)
  pub const K_DATA_ALIGN_FACTOR: i32 = 8;

  pub(crate) const K_RAW_DATA_LIMIT: u32 = 1024;
}

impl Default for UnwindBuilderDwarf2 {
  fn default() -> Self {
    unsafe {
      let mut builder = Self {
        base: zeroed(),
        begin_offset: 0,
        unwind_functions: Vec::new(),
        raw_data: [0; 1024],
        pos: null_mut(),
        fde_entry_start: null_mut(),
      };
      builder.pos = builder.raw_data.as_mut_ptr();
      builder
    }
  }
}
