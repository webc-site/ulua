use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_reg_alloc_x_64::IrRegAllocX64};

impl IrRegAllocX64 {
  pub fn assert_all_free(&self) {
    for reg in Self::K_GPR_ALLOC_ORDER {
      CODEGEN_ASSERT!(self.free_gpr_map[reg.index() as usize]);
    }

    for free in &self.free_xmm_map {
      CODEGEN_ASSERT!(*free);
    }
  }
}
