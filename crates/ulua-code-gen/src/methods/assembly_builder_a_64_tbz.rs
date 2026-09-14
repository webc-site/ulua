use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn tbz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
    self.place_btr("tbz", label, 0b0110110, src, bit);
  }
}
