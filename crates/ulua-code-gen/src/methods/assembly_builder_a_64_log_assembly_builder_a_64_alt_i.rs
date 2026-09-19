use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn log_c_char_register_a_64_label_i32(
    &mut self,
    opcode: &str,
    src: RegisterA64,
    label: Label,
    imm: i32,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_register_a_64(src);
    self.text.push(',');
    if imm >= 0 {
      self.log_append(format_args!("#{},", imm));
    }
    self.log_append(format_args!(".L{}\n", label.id));
  }
}
