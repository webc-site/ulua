use crate::records::{register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64};

impl ScopedRegX64 {
  pub fn scoped_reg_x_64_scoped_reg_x_64(&mut self) {
    if self.reg != RegisterX64::NOREG {
      unsafe { &mut *self.owner }.free_reg(self.reg);
    }
  }
}
