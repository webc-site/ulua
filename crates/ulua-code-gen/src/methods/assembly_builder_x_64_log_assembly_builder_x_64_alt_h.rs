use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, label::Label, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn log_c_char_register_x_64_label(&mut self, opcode: &str, reg: RegisterX64, label: Label) {
    // C++ inlines this (does NOT call the 2-operand `log`, which would append a
    // trailing newline after the register): pad opcode, log the register operand
    // with no newline, then `,.l<id>\n`.
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand_x_64(reg.into());
    self.text.push(',');
    self.log_append(format_args!(".L{}\n", label.id));
  }
}
