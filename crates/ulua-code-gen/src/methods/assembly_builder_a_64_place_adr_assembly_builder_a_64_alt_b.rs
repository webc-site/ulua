use crate::{
  enums::{kind::Kind, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn place_adr_c_char_register_a_64_u8_label(
    &mut self,
    name: &str,
    dst: RegisterA64,
    op: u8,
    label: &mut Label,
  ) {
    assert!(dst.kind() == KindA64::X);

    self.place(dst.index() as u32 | ((op as u32) << 24));
    self.commit();

    self.patch_label(label, Kind::IMM19);

    if self.log_text {
      // C++ logs `log(name, dst, label)`; the imm parameter defaults to -1 (suppressed).
      self.log_c_char_register_a_64_label_i32(name, dst, *label, -1);
    }
  }
}
