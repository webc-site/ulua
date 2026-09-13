use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64},
};

impl ScopedRegX64 {
  pub fn take(&mut self, reg: RegisterX64) {
    CODEGEN_ASSERT!(self.reg.register_x_64_operator_eq(RegisterX64::NOREG));
    self.reg = unsafe { (*self.owner).take_reg(reg, 0xffffffff) };
  }
}
