use crate::records::remove_dead_store_state::RemoveDeadStoreState;
impl RemoveDeadStoreState {
  pub fn use_varargs(&mut self, vararg_start: u8) {
    let max_reg: u8 = 255;
    for i in vararg_start..=max_reg {
      self.use_reg(i);
    }
  }
}
