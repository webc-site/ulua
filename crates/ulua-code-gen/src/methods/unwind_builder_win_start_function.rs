use crate::records::{
  register_x_64::RegisterX64, unwind_builder_win::UnwindBuilderWin,
  unwind_function_win::UnwindFunctionWin,
};

impl UnwindBuilderWin {
  pub fn start_function(&mut self) {
    // End offset is filled in later and everything gets adjusted at the end
    let func = UnwindFunctionWin {
      begin_offset: 0,
      end_offset: 0,
      unwind_info_offset: unsafe { self.raw_data_pos.offset_from(self.raw_data.as_ptr()) as u32 },
    };
    self.unwind_functions.push(func);

    self.unwind_codes.clear();
    self.unwind_codes.reserve(16);

    self.prolog_size = 0;

    // rax has register index 0, which in Windows unwind info means that frame register is not used
    self.frame_reg = RegisterX64::RAX;
    self.frame_reg_offset = 0;
  }
}
