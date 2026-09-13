use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_register_a_64(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src: RegisterA64,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_register_a_64(dst);
    self.text.push(',');
    self.log_register_a_64(src);
    self.text.push('\n');
  }
}
