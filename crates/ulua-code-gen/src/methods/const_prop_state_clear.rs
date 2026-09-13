use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn clear_const_prop_state(&mut self) {
    self.clear();
  }
}
