use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  pub fn indent(&mut self) {
    self.indentation += 4;
  }
}
