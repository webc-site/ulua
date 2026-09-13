use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn pop_comma(&mut self, c: bool) {
    self.comma = c;
  }
}
