use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64},
};

impl ScopedRegX64 {
  pub fn free(&mut self) {
    CODEGEN_ASSERT!(self.reg != RegisterX64::NOREG, "ScopedRegX64::free");
    unsafe { &mut *self.owner }.free_reg(self.reg);
    self.reg = RegisterX64::NOREG;
  }
}
