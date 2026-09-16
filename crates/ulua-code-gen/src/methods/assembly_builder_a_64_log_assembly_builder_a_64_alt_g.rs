use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_i32_i32(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src: i32,
    shift: i32,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_register_a_64(dst);
    self.text.push(',');
    self.log_append(format_args!("#{}", src));
    if shift > 0 {
      self.log_append(format_args!(" LSL #{}", shift));
    }
    self.text.push('\n');
  }
}
