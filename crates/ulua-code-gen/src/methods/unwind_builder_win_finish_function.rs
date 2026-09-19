use core::{mem::size_of, ptr::copy_nonoverlapping};

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    unwind_builder_win::UnwindBuilderWin, unwind_code_win::UnwindCodeWin,
    unwind_info_win::UnwindInfoWin,
  },
};

impl UnwindBuilderWin {
  pub fn finish_function(&mut self, begin_offset: u32, end_offset: u32) {
    let last = self
      .unwind_functions
      .last_mut()
      .expect("finish_function without start_function");
    last.begin_offset = begin_offset;
    last.end_offset = end_offset;

    CODEGEN_ASSERT!(self.unwind_codes.len() < 256);

    let mut info = UnwindInfoWin::default();
    info.set_version(1);
    info.set_flags(0);
    info.prologsize = self.prolog_size;
    info.unwindcodecount = self.unwind_codes.len() as u8;

    CODEGEN_ASSERT!(self.frame_reg.index() < 16);
    info.set_framereg(self.frame_reg.index());

    CODEGEN_ASSERT!(self.frame_reg_offset < 16);
    info.set_frameregoff(self.frame_reg_offset);

    unsafe {
      let raw_end = self
        .raw_data
        .as_mut_ptr()
        .add(UnwindBuilderWin::K_RAW_DATA_LIMIT as usize);
      CODEGEN_ASSERT!(self.raw_data_pos.add(size_of::<UnwindInfoWin>()) <= raw_end);

      copy_nonoverlapping(
        &info as *const UnwindInfoWin as *const u8,
        self.raw_data_pos,
        size_of::<UnwindInfoWin>(),
      );
      self.raw_data_pos = self.raw_data_pos.add(size_of::<UnwindInfoWin>());

      if !self.unwind_codes.is_empty() {
        let mut unwind_code_pos = self
          .raw_data_pos
          .add(size_of::<UnwindCodeWin>() * (self.unwind_codes.len() - 1));
        CODEGEN_ASSERT!(unwind_code_pos <= raw_end);

        for code in &self.unwind_codes {
          copy_nonoverlapping(
            code as *const UnwindCodeWin as *const u8,
            unwind_code_pos,
            size_of::<UnwindCodeWin>(),
          );
          unwind_code_pos = unwind_code_pos.sub(size_of::<UnwindCodeWin>());
        }
      }

      self.raw_data_pos = self
        .raw_data_pos
        .add(size_of::<UnwindCodeWin>() * self.unwind_codes.len());

      if !self.unwind_codes.len().is_multiple_of(2) {
        self.raw_data_pos = self.raw_data_pos.add(size_of::<UnwindCodeWin>());
      }

      CODEGEN_ASSERT!(self.raw_data_pos <= raw_end);
    }
  }
}
