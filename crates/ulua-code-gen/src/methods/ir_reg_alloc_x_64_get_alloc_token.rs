use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;

impl IrRegAllocX64 {
  pub fn get_alloc_token(&self) -> u32 {
    self.alloc_action_count
  }
}
