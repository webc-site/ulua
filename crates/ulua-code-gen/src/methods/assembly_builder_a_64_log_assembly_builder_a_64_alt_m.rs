use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn log_label(&mut self, label: Label) {
    self.log_append(format_args!(".L{}:\n", label.id));
  }
}
