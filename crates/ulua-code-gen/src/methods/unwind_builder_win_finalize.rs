use core::{ffi::c_void, mem::size_of, ptr};

use crate::records::{
  unwind_builder_win::UnwindBuilderWin, unwind_function_win::UnwindFunctionWin,
};

impl UnwindBuilderWin {
  pub fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    _func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    let mut current_target = target;
    let k_full_block_function: u32 = 0xFFFFFFFF;

    for func in &self.unwind_functions {
      let mut adjusted_func = *func;

      adjusted_func.begin_offset += offset as u32;

      if adjusted_func.end_offset == k_full_block_function {
        adjusted_func.end_offset = block_size as u32;
      } else {
        adjusted_func.end_offset += offset as u32;
      }

      adjusted_func.unwind_info_offset +=
        (size_of::<UnwindFunctionWin>() * self.unwind_functions.len()) as u32;

      unsafe {
        ptr::copy_nonoverlapping(
          &adjusted_func as *const UnwindFunctionWin as *const u8,
          current_target,
          size_of::<UnwindFunctionWin>(),
        );
        current_target = current_target.add(size_of::<UnwindFunctionWin>());
      }
    }

    let raw_data_len = unsafe { self.raw_data_pos.offset_from(self.raw_data.as_ptr()) } as usize;
    unsafe {
      ptr::copy_nonoverlapping(self.raw_data.as_ptr(), current_target, raw_data_len);
    }

    self.unwind_functions.len()
  }
}
