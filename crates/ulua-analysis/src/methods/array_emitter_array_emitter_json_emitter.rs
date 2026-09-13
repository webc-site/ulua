use crate::records::{array_emitter::ArrayEmitter, json_emitter::JsonEmitter};

impl ArrayEmitter {
  pub fn array_emitter(&mut self, emitter: &mut JsonEmitter) {
    self.comma = emitter.push_comma();
    emitter.write_raw_string_view("[");
    self.emitter = emitter as *mut JsonEmitter;
    self.finished = false;
  }
}
