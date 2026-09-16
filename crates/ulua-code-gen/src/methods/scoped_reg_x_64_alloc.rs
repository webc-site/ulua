use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_data::K_INVALID_INST_IDX, register_x_64::RegisterX64, scoped_reg_x_64::ScopedRegX64,
  },
};

impl ScopedRegX64 {
  pub fn alloc(&mut self, size: SizeX64) {
    CODEGEN_ASSERT!(self.reg == RegisterX64::NOREG);
    let owner = unsafe { &mut *self.owner };
    self.reg = owner.alloc_reg(size, K_INVALID_INST_IDX);
  }
}
