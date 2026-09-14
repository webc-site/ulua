use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::ir_reg_alloc_x_64::IrRegAllocX64};

impl IrRegAllocX64 {
  pub fn assert_no_spills(&self) {
    CODEGEN_ASSERT!(self.spills.is_empty());
  }
}
