use alloc::string::String;

use crate::{
  methods::json_emitter_write_raw_json_emitter::CHUNK_SIZE, records::json_emitter::JsonEmitter,
};

impl JsonEmitter {
  pub fn new_chunk(&mut self) {
    self.chunks.push(String::with_capacity(CHUNK_SIZE));
  }
}
