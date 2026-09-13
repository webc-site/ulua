use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn new_chunk(&mut self) {
    self.chunks.push(String::new());
    if let Some(last_chunk) = self.chunks.last_mut() {
      last_chunk.reserve(4096);
    }
  }
}
