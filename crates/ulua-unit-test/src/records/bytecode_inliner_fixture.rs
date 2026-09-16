use alloc::string::String;
#[derive(Debug, Clone)]
pub struct BytecodeInlinerFixture {
  pub strings: Vec<String>,
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
