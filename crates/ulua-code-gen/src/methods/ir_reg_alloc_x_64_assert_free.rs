use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64},
};
impl IrRegAllocX64 {
  /// cpp IrRegAllocX64.h:91 声明的调试辅助方法；cpp 内部亦无调用点，保留 API 表面。
  pub fn assert_free(&self, reg: RegisterX64) {
    if reg.size() == SizeX64::Xmmword {
      CODEGEN_ASSERT!(self.free_xmm_map[reg.index() as usize]);
    } else {
      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
    }
  }
}
