use crate::{
  enums::{kind::Kind, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn place_btr(
    &mut self,
    name: &str,
    name_inv: &str,
    label: &mut Label,
    op: u8,
    cond: RegisterA64,
    bit: u8,
  ) {
    debug_assert!(
      cond.kind() == KindA64::X || cond.kind() == KindA64::W,
      "cond.kind() == KindA64::x || cond.kind() == KindA64::w"
    );
    debug_assert!(
      bit < (if cond.kind() == KindA64::X { 64 } else { 32 }),
      "bit < (cond.kind() == KindA64::x ? 64 : 32)"
    );

    self.place(
      cond.index() as u32
        | (((bit & 0x1f) as u32) << 19)
        | ((op as u32) << 24)
        | (((bit >> 5) as u32) << 31),
    );
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1614-1649：
    // FarRefs 开启时 tbz/tbnz 经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM14, 24);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_c_char_register_a_64_label_i32(name_inv, cond, skip_label, bit as i32);
        self.log_c_char_label("b", *label);
        self.log_label(skip_label);
      } else {
        self.log_c_char_register_a_64_label_i32(name, cond, *label, bit as i32);
      }
    }
  }
}
