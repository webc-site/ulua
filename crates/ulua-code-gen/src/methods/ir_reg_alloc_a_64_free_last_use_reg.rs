use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_inst::IrInst, ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64},
};

impl IrRegAllocA64 {
  pub fn free_last_use_reg(&mut self, target: &mut IrInst, index: u32) {
    if target.last_use == index && !target.reused_reg {
      CODEGEN_ASSERT!(!target.spilled && !target.needs_reload);

      // Register might have already been freed if it had multiple uses inside a single instruction
      if target.reg_a64 == RegisterA64::NOREG {
        return;
      }

      self.free_reg(target.reg_a64);
      target.reg_a64 = RegisterA64::NOREG;
    }
  }
}
