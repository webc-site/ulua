use alloc::{string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct BytecodeCompilerFixture {
  pub strings: Vec<String>,
}

impl BytecodeCompilerFixture {
  pub fn new() -> Self {
    Self {
      strings: Vec::new(),
    }
  }
}

impl Default for BytecodeCompilerFixture {
  fn default() -> Self {
    Self::new()
  }
}
