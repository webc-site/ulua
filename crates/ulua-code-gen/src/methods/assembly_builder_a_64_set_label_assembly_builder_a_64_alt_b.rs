use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn set_label_label(&mut self, label: &mut Label) {
    if label.id == 0 {
      label.id = self.next_label;
      self.next_label = self.next_label.wrapping_add(1);
      self.label_locations.push(!0u32);
    }

    label.location = self.get_code_size();
    self.label_locations[(label.id - 1) as usize] = label.location;

    if self.log_text {
      self.log_label(*label);
    }
  }
}
