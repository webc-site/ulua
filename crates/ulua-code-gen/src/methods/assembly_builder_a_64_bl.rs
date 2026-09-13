use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label};

impl AssemblyBuilderA64 {
  pub fn bl(&mut self, label: &mut Label) {
    self.place_b("bl", label, 0b1_00101);
  }
}
