use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64},
};
impl IrRegAllocX64 {
  pub fn assert_free(&self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(self.free_xmm_map[reg.index() as usize]);
    } else {
      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
    }
  }
}
