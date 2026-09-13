use crate::records::{const_prop_state::ConstPropState, register_info::RegisterInfo};
impl ConstPropState {
  pub fn invalidate_captured_registers(&mut self) {
    let max_reg = self.max_reg;
    let captured_regs = unsafe { &(*self.function).cfg.captured.regs };
    for i in 0..=max_reg {
      let reg = i as usize;
      if (captured_regs[reg / 64] & (1u64 << (reg % 64))) != 0 {
        let idx = i as usize;
        let reg_ptr: *mut RegisterInfo = &mut self.regs[idx];
        unsafe {
          self.invalidate_register_info_bool_bool(&mut *reg_ptr, true, true);
        }
      }
    }
  }
}
