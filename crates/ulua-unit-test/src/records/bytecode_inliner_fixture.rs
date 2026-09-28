use alloc::vec::Vec;
#[derive(Debug, Clone)]
pub struct BytecodeInlinerFixture {
  pub strings: Vec<Vec<u8>>,
}

impl BytecodeInlinerFixture {
  pub fn new() -> Self {
    Self {
      strings: Vec::new(),
    }
  }
}

impl Default for BytecodeInlinerFixture {
  fn default() -> Self {
    Self::new()
  }
}
