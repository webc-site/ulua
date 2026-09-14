use crate::records::remove_dead_store_state::RemoveDeadStoreState;

impl RemoveDeadStoreState {
  pub fn def_varargs(&mut self, vararg_start: u8) {
    let max_reg = 255;
    for i in vararg_start..=max_reg {
      self.def_reg(i);
    }
  }
}
