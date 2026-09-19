use crate::records::{ir_inst::IrInst, ir_reg_alloc_x_64::IrRegAllocX64};

impl IrRegAllocX64 {
  pub fn is_last_use_reg(&self, target: &IrInst, inst_idx: u32) -> bool {
    target.last_use == inst_idx && !target.reused_reg
  }
}
