use crate::{
  enums::size_x_64::SizeX64,
  records::{
    ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

impl ScopedRegX64 {
  pub fn scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(
    &mut self,
    owner: &mut IrRegAllocX64,
    size: SizeX64,
  ) {
    self.owner = owner as *mut IrRegAllocX64;
    self.reg = RegisterX64::NOREG;
    self.alloc(size);
  }
}
