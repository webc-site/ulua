use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// cpp: `BytecodeBuilder::getFunctionCount`
  /// (`cpp/Bytecode/include/Luau/BytecodeBuilder.h:173`)
  pub fn get_function_count(&self) -> u32 {
    self.functions.len() as u32
  }
}
