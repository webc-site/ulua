use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label},
};

/// cpp placeBC 日志用：textForCondition[cond ^ 1]，即反转条件的助记符
const fn text_for_condition_inverted(cond: u8) -> &'static str {
  match cond ^ 1 {
    0 => "b.eq",
    1 => "b.ne",
    2 => "b.cs",
    3 => "b.cc",
    4 => "b.mi",
    5 => "b.pl",
    6 => "b.vs",
    7 => "b.vc",
    8 => "b.hi",
    9 => "b.ls",
    10 => "b.ge",
    11 => "b.lt",
    12 => "b.gt",
    13 => "b.le",
    14 => "b.al",
    _ => "b",
  }
}

impl AssemblyBuilderA64 {
  pub fn place_bc(&mut self, name: &str, label: &mut Label, op: u8, cond: u8) {
    self.place(cond as u32 | ((op as u32) << 24));
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1544-1567：
    // FarRefs 开启时条件分支经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM19, 0);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_c_char_label(text_for_condition_inverted(cond), skip_label);
        self.log_c_char_label("b", *label);
        self.log_label(skip_label);
      } else {
        self.log_c_char_label(name, *label);
      }
    }
  }
}
