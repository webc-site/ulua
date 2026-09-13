use alloc::vec::Vec;
use core::{mem::zeroed, ptr::null_mut};

use crate::records::{
  register_x_64::RegisterX64, unwind_builder::UnwindBuilder, unwind_code_win::UnwindCodeWin,
  unwind_function_win::UnwindFunctionWin,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderWin {
  pub base: UnwindBuilder,
  pub(crate) begin_offset: usize,
  pub(crate) raw_data: [u8; 1024],
  pub(crate) raw_data_pos: *mut u8,
  pub(crate) unwind_functions: Vec<UnwindFunctionWin>,
  pub(crate) unwind_codes: Vec<UnwindCodeWin>,
  pub(crate) prolog_size: u8,
  pub(crate) frame_reg: RegisterX64,
  pub(crate) frame_reg_offset: u8,
}

impl UnwindBuilderWin {
  pub(crate) const K_RAW_DATA_LIMIT: u32 = 1024;
}

impl Default for UnwindBuilderWin {
  fn default() -> Self {
    unsafe {
      let mut builder = Self {
        base: zeroed(),
        begin_offset: 0,
        raw_data: [0; 1024],
        raw_data_pos: null_mut(),
        unwind_functions: Vec::new(),
        unwind_codes: Vec::new(),
        prolog_size: 0,
        frame_reg: zeroed(), // X64::noreg
        frame_reg_offset: 0,
      };
      builder.raw_data_pos = builder.raw_data.as_mut_ptr();
      builder
    }
  }
}
