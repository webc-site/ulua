use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// cpp: `BytecodeBuilder::clearStrings`
  /// (`cpp/Bytecode/include/Luau/BytecodeBuilder.h:182`)
  pub fn clear_strings(&mut self) {
    self.debug_strings.clear();
    self.string_table.clear();
  }
}
