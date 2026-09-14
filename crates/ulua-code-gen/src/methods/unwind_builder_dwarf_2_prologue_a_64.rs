use crate::{
  enums::kind_a_64::KindA64,
  functions::{
    advance_location::advance_location, define_cfa_expression_offset::define_cfa_expression_offset,
    define_saved_register_location::define_saved_register_location,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{register_a_64::RegisterA64, unwind_builder_dwarf_2::UnwindBuilderDwarf2},
};

impl UnwindBuilderDwarf2 {
  pub fn prologue_a_64(&mut self, prologue_size: u32, stack_size: u32, regs: &[RegisterA64]) {
    unsafe {
      CODEGEN_ASSERT!(stack_size.is_multiple_of(16));
      CODEGEN_ASSERT!(
        regs.len() >= 2
          && (*regs.get_unchecked(0)).index() == 29
          && (*regs.get_unchecked(1)).index() == 30
      );
      CODEGEN_ASSERT!((regs.len() as u32) * 8 <= stack_size);

      self.pos = advance_location(self.pos, 4);
      self.pos = define_cfa_expression_offset(self.pos, stack_size);

      self.pos = advance_location(self.pos, prologue_size - 4);

      for i in 0..regs.len() {
        let reg = regs.get_unchecked(i);
        CODEGEN_ASSERT!((*reg).kind() == KindA64::X);
        self.pos = define_saved_register_location(
          self.pos,
          (*reg).index() as i32,
          stack_size - (i as u32 * 8),
        );
      }
    }
  }
}
