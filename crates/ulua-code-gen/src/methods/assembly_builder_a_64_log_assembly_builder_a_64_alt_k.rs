use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn log_c_char_label(&mut self, opcode: &str, label: Label) {
    self.log_append(format_args!(" {:<12}.L{}\n", opcode, label.id));
  }
}
