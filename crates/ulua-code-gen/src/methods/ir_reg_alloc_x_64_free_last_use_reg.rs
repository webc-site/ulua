use crate::records::{
  ir_inst::IrInst, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
};

impl IrRegAllocX64 {
  pub fn free_last_use_reg(&mut self, target: &mut IrInst, inst_idx: u32) {
    if self.is_last_use_reg(target, inst_idx) {
      debug_assert!(!target.spilled && !target.needs_reload);

      // Register might have already been freed if it had multiple uses inside a single instruction
      if target.reg_x64 == RegisterX64::NOREG {
        return;
      }

      self.free_reg(target.reg_x64);
      target.reg_x64 = RegisterX64::NOREG;
    }
  }
}
