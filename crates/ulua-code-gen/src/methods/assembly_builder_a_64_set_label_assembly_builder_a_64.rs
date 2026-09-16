use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn set_label(&mut self) -> Label {
    let label = Label {
      id: self.next_label,
      location: self.get_code_size(),
    };
    self.next_label = self.next_label.wrapping_add(1);
    self.label_locations.push(!0u32);

    if self.log_text {
      self.log_label(label);
    }

    label
  }
}
