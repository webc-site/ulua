use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
impl IrRegAllocA64 {
  pub fn get_alloc_token(&self) -> u32 {
    self.alloc_action_count
  }
}
