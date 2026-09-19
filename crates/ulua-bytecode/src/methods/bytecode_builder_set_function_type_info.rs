use alloc::vec::Vec;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn set_function_type_info(&mut self, value: Vec<u8>) {
    self.functions[self.current_function as usize].typeinfo = value;
  }
}
