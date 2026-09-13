use alloc::string::String;

use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn str(&mut self) -> String {
    self.chunks.join("")
  }
}
