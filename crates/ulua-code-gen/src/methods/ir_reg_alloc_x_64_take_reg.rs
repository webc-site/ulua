use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_data::K_INVALID_INST_IDX, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
  },
};

impl IrRegAllocX64 {
  pub fn take_reg(&mut self, reg: RegisterX64, inst_idx: u32) -> RegisterX64 {
    if reg.size() == SizeX64::Xmmword {
      if !self.free_xmm_map[reg.index() as usize] {
        CODEGEN_ASSERT!(self.xmm_inst_users[reg.index() as usize] != K_INVALID_INST_IDX);
        let inst_ptr = unsafe {
          let instructions = &mut (*self.function).instructions;
          &mut instructions[self.xmm_inst_users[reg.index() as usize] as usize]
        };
        self.preserve(inst_ptr);
      }

      CODEGEN_ASSERT!(self.free_xmm_map[reg.index() as usize]);
      self.free_xmm_map[reg.index() as usize] = false;
      self.xmm_inst_users[reg.index() as usize] = inst_idx;
    } else {
      if !self.free_gpr_map[reg.index() as usize] {
        CODEGEN_ASSERT!(self.gpr_inst_users[reg.index() as usize] != K_INVALID_INST_IDX);
        let inst_ptr = unsafe {
          let instructions = &mut (*self.function).instructions;
          &mut instructions[self.gpr_inst_users[reg.index() as usize] as usize]
        };
        self.preserve(inst_ptr);
      }

      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
      self.free_gpr_map[reg.index() as usize] = false;
      self.gpr_inst_users[reg.index() as usize] = inst_idx;
    }

    reg
  }
}
