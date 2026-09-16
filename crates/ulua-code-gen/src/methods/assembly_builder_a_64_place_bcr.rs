use crate::{
  enums::{kind::Kind, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn place_bcr(
    &mut self,
    name: &str,
    name_inv: &str,
    label: &mut Label,
    op: u8,
    cond: RegisterA64,
  ) {
    assert!(cond.kind() == KindA64::W || cond.kind() == KindA64::X);

    let sf = if cond.kind() == KindA64::X {
      0x8000_0000
    } else {
      0
    };

    self.place(cond.index() as u32 | ((op as u32) << 24) | sf);
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1574-1604：
    // FarRefs 开启时 cbz/cbnz 经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM19, 24);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_c_char_register_a_64_label_i32(name_inv, cond, skip_label, -1);
        self.log_c_char_label("b", *label);
        self.log_label(skip_label);
      } else {
        // C++ logs `log(name, cond, label)`; the imm parameter defaults to -1 (suppressed).
        self.log_c_char_register_a_64_label_i32(name, cond, *label, -1);
      }
    }
  }
}
