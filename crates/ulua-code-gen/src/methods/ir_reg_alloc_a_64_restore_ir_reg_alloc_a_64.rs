use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_reg_alloc_a_64::IrRegAllocA64};

impl IrRegAllocA64 {
  pub fn restore_usize(&mut self, start: usize) {
    CODEGEN_ASSERT!(start <= self.spills.len());

    if start < self.spills.len() {
      let mut i = start;
      while i < self.spills.len() {
        let s = self.spills[i]; // copy in case takeReg reallocates spills
        let reg = self.take_reg(s.origin, s.inst);

        self.restore_ir_reg_alloc_a_64_spill_register_a_64(&s, reg);

        i += 1;
      }

      self.spills.truncate(start);
    }
  }
}
