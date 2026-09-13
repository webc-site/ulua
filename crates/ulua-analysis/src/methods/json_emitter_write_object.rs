use crate::records::{json_emitter::JsonEmitter, object_emitter::ObjectEmitter};

impl JsonEmitter {
  pub fn write_object(&mut self) -> ObjectEmitter {
    let comma = self.push_comma();
    self.write_raw_string_view("{");

    ObjectEmitter {
      emitter: self as *mut JsonEmitter,
      comma,
      finished: false,
    }
  }
}
