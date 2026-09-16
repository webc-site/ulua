use crate::records::{json_emitter::JsonEmitter, object_emitter::ObjectEmitter};

impl ObjectEmitter {
  pub fn object_emitter(&mut self, emitter: &mut JsonEmitter) {
    self.comma = emitter.push_comma();
    emitter.write_raw_string_view("{");
    self.emitter = emitter as *mut JsonEmitter;
    self.finished = false;
  }
}
