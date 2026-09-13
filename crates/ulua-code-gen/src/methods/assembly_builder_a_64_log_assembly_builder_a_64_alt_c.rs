use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_register_a_64_i32(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: i32,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));

    let xzr = RegisterA64::XZR;
    let wzr = RegisterA64::WZR;

    if !dst.register_a_64_operator_eq(xzr) && !dst.register_a_64_operator_eq(wzr) {
      self.log_register_a_64(dst);
      self.text.push(',');
    }

    self.log_register_a_64(src1);
    self.text.push(',');
    self.log_append(format_args!("#{}", src2));
    self.text.push('\n');
  }
}
