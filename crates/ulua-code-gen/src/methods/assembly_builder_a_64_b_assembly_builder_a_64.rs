use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn b_label(&mut self, label: &mut Label) {
    self.place_b("b", label, 0b0_00101);
  }
}
