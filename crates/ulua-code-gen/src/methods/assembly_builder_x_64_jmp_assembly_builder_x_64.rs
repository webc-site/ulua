use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label};

impl AssemblyBuilderX64 {
  pub fn jmp_label(&mut self, label: &mut Label) {
    self.place(0xe9);
    self.place_label(label);

    if self.log_text {
      self.log_c_char_label("jmp", *label);
    }

    self.commit();
  }
}
