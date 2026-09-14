use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn tbnz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
    self.place_btr("tbnz", label, 0b0110111, src, bit);
  }
}
