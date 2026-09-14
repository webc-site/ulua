use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn log_c_char(&mut self, opcode: &str) {
    self.log_append(format_args!(" {}\n", opcode));
  }
}
