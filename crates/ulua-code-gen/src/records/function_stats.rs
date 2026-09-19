extern crate alloc;

use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionStats {
  pub name: String,
  pub line: i32,
  pub bcode_count: u32,
  pub ir_count: u32,
  pub asm_count: u32,
  pub asm_size: u32,
  pub bytecode_summary: Vec<Vec<u32>>,
}

impl Default for FunctionStats {
  fn default() -> Self {
    Self {
      name: String::new(),
      line: -1,
      bcode_count: 0,
      ir_count: 0,
      asm_count: 0,
      asm_size: 0,
      bytecode_summary: Vec::new(),
    }
  }
}
