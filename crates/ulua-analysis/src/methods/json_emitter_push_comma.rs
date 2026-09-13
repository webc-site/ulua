use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn push_comma(&mut self) -> bool {
    let current = self.comma;
    self.comma = false;
    current
  }
}
