use crate::records::{json_emitter::JsonEmitter, object_emitter::ObjectEmitter};

impl JsonEmitter {
  pub fn write_object(&mut self) -> ObjectEmitter<'_> {
    let comma = self.push_comma();
    self.write_raw_string_view("{");

    ObjectEmitter {
      emitter: self,
      comma,
      finished: false,
    }
  }
}
