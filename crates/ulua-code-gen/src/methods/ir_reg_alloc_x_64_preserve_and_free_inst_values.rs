use crate::records::{ir_data::K_INVALID_INST_IDX, ir_reg_alloc_x_64::IrRegAllocX64};

impl IrRegAllocX64 {
  pub fn preserve_and_free_inst_values(&mut self) {
    for inst_idx in self.gpr_inst_users {
      if inst_idx != K_INVALID_INST_IDX {
        let inst = unsafe {
          let instructions = &mut (*self.function).instructions;
          &mut instructions[inst_idx as usize]
        };
        self.preserve(inst);
      }
    }

    for inst_idx in self.xmm_inst_users {
      if inst_idx != K_INVALID_INST_IDX {
        let inst = unsafe {
          let instructions = &mut (*self.function).instructions;
          &mut instructions[inst_idx as usize]
        };
        self.preserve(inst);
      }
    }
  }
}
