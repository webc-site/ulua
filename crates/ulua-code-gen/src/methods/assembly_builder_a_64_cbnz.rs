use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn cbnz(&mut self, src: RegisterA64, label: &mut Label) {
    self.place_bcr("cbnz", label, 0b0110101, src);
  }
}
