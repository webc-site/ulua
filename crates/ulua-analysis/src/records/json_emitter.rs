//! Source: `Analysis/include/Luau/JsonEmitter.h` (hand-ported; fields only)

use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone)]
pub struct JsonEmitter {
  pub comma: bool,
  pub chunks: Vec<String>,
}

impl Default for JsonEmitter {
  fn default() -> Self {
    let mut emitter = Self {
      comma: false,
      chunks: Vec::new(),
    };
    emitter.new_chunk();
    emitter
  }
}
