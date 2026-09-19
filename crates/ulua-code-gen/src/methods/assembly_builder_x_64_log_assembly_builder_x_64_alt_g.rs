use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label};

impl AssemblyBuilderX64 {
  pub fn log_c_char_label(&mut self, opcode: &str, label: Label) {
    self.log_append(format_args!(" {:<12}.L{}\n", opcode, label.id));
  }
}
