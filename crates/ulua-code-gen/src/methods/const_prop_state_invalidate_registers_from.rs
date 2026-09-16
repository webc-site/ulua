use crate::records::{const_prop_state::ConstPropState, register_info::RegisterInfo};
impl ConstPropState {
  pub fn invalidate_registers_from(&mut self, first_reg: i32) {
    for i in first_reg..=self.max_reg {
      let idx = i as usize;
      // Avoid borrowing `self` (and `self.regs[idx]`) mutably more than once:
      // - take a raw pointer to the register slot
      // - perform the invalidation call using the raw pointer as a mutable ref
      let reg_ptr: *mut RegisterInfo = &mut self.regs[idx];
      unsafe {
        self.invalidate_register_info_bool_bool(&mut *reg_ptr, true, true);
      }
    }
  }
}
