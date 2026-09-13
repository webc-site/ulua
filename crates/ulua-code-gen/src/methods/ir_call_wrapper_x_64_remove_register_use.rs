use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_call_wrapper_x_64::IrCallWrapperX64, register_x_64::RegisterX64},
};

impl IrCallWrapperX64 {
  pub fn remove_register_use(&mut self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(self.xmm_uses[reg.index() as usize] != 0);
      self.xmm_uses[reg.index() as usize] -= 1;

      if self.xmm_uses[reg.index() as usize] == 0 {
        unsafe {
          (*self.regs).free_reg(reg);
        }
      }
    } else if reg.size() != SizeX64::None {
      CODEGEN_ASSERT!(self.gpr_uses[reg.index() as usize] != 0);
      self.gpr_uses[reg.index() as usize] -= 1;

      if self.gpr_uses[reg.index() as usize] == 0 {
        unsafe {
          if (*self.regs).should_free_gpr(reg) {
            (*self.regs).free_reg(reg);
          }
        }
      }
    }
  }
}
