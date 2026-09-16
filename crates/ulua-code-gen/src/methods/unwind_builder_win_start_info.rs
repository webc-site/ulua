use crate::{
  enums::arch::Arch,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{register_x_64::RegisterX64, unwind_builder_win::UnwindBuilderWin},
};

impl UnwindBuilderWin {
  pub fn start_info(&mut self, arch: Arch) {
    CODEGEN_ASSERT!(arch == Arch::X64);

    self.begin_offset = 0;
    self.raw_data_pos = self.raw_data.as_mut_ptr();
    self.unwind_functions.clear();
    self.unwind_codes.clear();
    self.prolog_size = 0;
    self.frame_reg = RegisterX64::RAX;
    self.frame_reg_offset = 0;
  }
}
