use core::mem::size_of;

use crate::records::{
  unwind_builder_win::UnwindBuilderWin, unwind_function_win::UnwindFunctionWin,
};

impl UnwindBuilderWin {
  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    let raw_data_ptr = self.raw_data.as_ptr();
    let raw_data_pos = self.raw_data_pos as *const u8;
    let raw_data_diff = unsafe { raw_data_pos.offset_from(raw_data_ptr) } as usize;

    size_of::<UnwindFunctionWin>() * self.unwind_functions.len() + raw_data_diff
  }
}
