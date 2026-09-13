use alloc::string::String;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_function_data(&self, id: u32) -> String {
    self.functions[id as usize].data.clone()
  }
}
