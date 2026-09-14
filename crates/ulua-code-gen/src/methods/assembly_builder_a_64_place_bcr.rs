use crate::{
  enums::{kind::Kind, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn place_bcr(&mut self, name: &str, label: &mut Label, op: u8, cond: RegisterA64) {
    assert!(cond.kind() == KindA64::W || cond.kind() == KindA64::X);

    let sf = if cond.kind() == KindA64::X {
      0x8000_0000
    } else {
      0
    };

    self.place(cond.index() as u32 | ((op as u32) << 24) | sf);
    self.commit();

    self.patch_label(label, Kind::IMM19);

    if self.log_text {
      // C++ logs `log(name, cond, label)`; the imm parameter defaults to -1 (suppressed).
      self.log_c_char_register_a_64_label_i32(name, cond, *label, -1);
    }
  }
}
