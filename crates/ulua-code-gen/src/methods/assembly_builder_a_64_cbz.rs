use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn cbz(&mut self, src: RegisterA64, label: &mut Label) {
    self.place_bcr("cbz", label, 0b011_0100, src);
  }
}
