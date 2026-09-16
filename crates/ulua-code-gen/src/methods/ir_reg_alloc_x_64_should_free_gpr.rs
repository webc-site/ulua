use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64},
};

impl IrRegAllocX64 {
  pub fn should_free_gpr(&self, reg: RegisterX64) -> bool {
    if reg.register_x_64_operator_eq(RegisterX64::NOREG) {
      return false;
    }

    CODEGEN_ASSERT!(reg.size() != SizeX64::Xmmword);

    for &gpr in &Self::K_GPR_ALLOC_ORDER {
      if reg.index() == gpr.index() {
        return true;
      }
    }

    false
  }
}
