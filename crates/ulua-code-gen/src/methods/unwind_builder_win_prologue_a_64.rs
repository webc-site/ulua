use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{register_a_64::RegisterA64, unwind_builder_win::UnwindBuilderWin},
};

impl UnwindBuilderWin {
  pub fn prologue_a_64(&mut self, _prologue_size: u32, _stack_size: u32, _regs: &[RegisterA64]) {
    CODEGEN_ASSERT!(false);
  }
}
