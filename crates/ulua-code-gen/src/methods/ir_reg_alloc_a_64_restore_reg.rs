use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_inst::IrInst, ir_reg_alloc_a_64::IrRegAllocA64},
};

impl IrRegAllocA64 {
  pub fn restore_reg(&mut self, inst: &mut IrInst) {
    let index = unsafe { &*self.function }.get_inst_index(inst);

    let mut i = 0;
    while i < self.spills.len() {
      if self.spills[i].inst == index {
        let s = self.spills[i]; // copy in case allocReg reallocates spills
        let reg = self.alloc_reg(s.origin.kind(), index);

        self.restore_ir_reg_alloc_a_64_spill_register_a_64(&s, reg);

        self.spills[i] = self.spills[self.spills.len() - 1];
        self.spills.pop();
        return;
      }
      i += 1;
    }

    CODEGEN_ASSERT!(false, "Expected to find a spill record");
  }
}
