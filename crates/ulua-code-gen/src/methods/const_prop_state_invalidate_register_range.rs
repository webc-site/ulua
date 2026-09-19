use crate::records::{const_prop_state::ConstPropState, register_info::RegisterInfo};
impl ConstPropState {
  pub fn invalidate_register_range(&mut self, first_reg: i32, count: i32) {
    if count == -1 {
      self.invalidate_registers_from(first_reg);
    } else {
      let max_reg = self.max_reg;
      for i in first_reg..(first_reg + count).min(max_reg + 1) {
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
}
