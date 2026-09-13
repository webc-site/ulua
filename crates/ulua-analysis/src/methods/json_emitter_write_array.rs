use crate::records::{array_emitter::ArrayEmitter, json_emitter::JsonEmitter};

impl JsonEmitter {
  pub fn write_array(&mut self) -> ArrayEmitter {
    let comma = self.push_comma();
    self.write_raw_string_view("[");

    ArrayEmitter {
      emitter: self as *mut JsonEmitter,
      comma,
      finished: false,
    }
  }
}
