use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label},
};

impl AssemblyBuilderA64 {
  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1870 patchLabelFar：
  /// 反向跳转超出 IMM19/IMM14 范围时，反转条件落到紧跟其后的 trampoline，
  /// 再经 IMM26 无条件跳转抵达远目标，并返回 skip label 供日志标注。
  pub fn patch_label_far(&mut self, label: &mut Label, kind: Kind, invert_bit: u32) -> Label {
    // 尚未放置的 label 仍按近跳转生成
    if label.location == !0u32 {
      self.patch_label(label, kind);
      return Label::default();
    }

    let location = self.get_code_size().wrapping_sub(1);

    // 检查反向跳转是否在范围内
    let value = label.location as i32 - location as i32;
    let range: i32 = if kind == Kind::IMM19 {
      1 << 19
    } else {
      1 << 14
    };

    if value > -(range >> 1) && value < (range >> 1) {
      self.patch_label(label, kind);
      return Label::default();
    }

    // 反转条件，跳到紧随其后的 trampoline
    self.code[location as usize] ^= 1u32 << invert_bit;
    self.patch_offset(location, 2, kind);

    // 放置更大范围的无条件跳转（与 placeB 相同编码但不打日志）
    self.place(0b0_00101 << 26);
    self.commit();

    self.patch_label(label, Kind::IMM26);

    let skip_label = Label {
      id: self.next_label,
      location: self.get_code_size(),
    };
    self.next_label = self.next_label.wrapping_add(1);
    self.label_locations.push(skip_label.location);
    skip_label
  }
}
