use std::vec::Vec;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_function_data(&self, id: u32) -> Vec<u8> {
    self.functions[id as usize].data.clone()
  }
}
