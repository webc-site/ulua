use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label};

impl AssemblyBuilderX64 {
  pub fn log_label(&mut self, label: Label) {
    self.log_append(format_args!(".L{}:\n", label.id));
  }
}
