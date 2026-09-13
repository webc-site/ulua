use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_data::K_INVALID_INST_IDX, ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64,
  },
};

impl IrRegAllocX64 {
  pub fn free_reg(&mut self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(!self.free_xmm_map[reg.index() as usize]);
      self.free_xmm_map[reg.index() as usize] = true;
      self.xmm_inst_users[reg.index() as usize] = K_INVALID_INST_IDX;
    } else {
      CODEGEN_ASSERT!(!self.free_gpr_map[reg.index() as usize]);
      self.free_gpr_map[reg.index() as usize] = true;
      self.gpr_inst_users[reg.index() as usize] = K_INVALID_INST_IDX;
    }
  }
}
