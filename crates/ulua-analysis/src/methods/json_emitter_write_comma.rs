use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn write_comma(&mut self) {
    if self.comma {
      self.write_raw_string_view(",");
    } else {
      self.comma = true;
    }
  }
}
