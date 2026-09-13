use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn log_c_char(&mut self, opcode: &str) {
    self.log_append(format_args!(" {}\n", opcode));
  }
}
