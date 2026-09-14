use crate::records::{
  assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64,
};

impl AssemblyBuilderA64 {
  pub fn adr_register_a_64_label(&mut self, dst: RegisterA64, label: &mut Label) {
    self.place_adr_c_char_register_a_64_u8_label("adr", dst, 0b10000, label);
  }
}
