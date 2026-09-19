use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn invalidate_user_call(&mut self) {
    self.invalidate_heap();
    self.invalidate_captured_registers();
    self.invalidate_value_propagation();

    self.in_safe_env = false;
  }
}

pub fn const_prop_state_invalidate_user_call(this: &mut ConstPropState) {
  this.invalidate_user_call();
}
