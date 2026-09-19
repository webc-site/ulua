use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

impl IrRegAllocA64 {
  pub fn free_temp_regs(&mut self) {
    CODEGEN_ASSERT!((self.gpr.free & self.gpr.temp) == 0);
    self.gpr.free |= self.gpr.temp;
    self.gpr.temp = 0;

    CODEGEN_ASSERT!((self.simd.free & self.simd.temp) == 0);
    self.simd.free |= self.simd.temp;
    self.simd.temp = 0;
  }
}
