use crate::records::stringifier_state::StringifierState;

impl StringifierState {
  pub fn dedent(&mut self) {
    self.indentation -= 4;
  }
}
