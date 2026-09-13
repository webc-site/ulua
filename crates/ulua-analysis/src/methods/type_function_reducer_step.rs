use crate::records::type_function_reducer::TypeFunctionReducer;

impl TypeFunctionReducer {
  pub fn step(&mut self) {
    if !self.queued_tys.empty() {
      self.step_type();
    } else if !self.queued_tps.empty() {
      self.step_pack();
    }
  }
}
