use crate::{
  enums::kind::Kind,
  records::{assembly_builder_a_64::AssemblyBuilderA64, label::Label},
};

impl AssemblyBuilderA64 {
  pub fn place_bc(&mut self, name: &str, label: &mut Label, op: u8, cond: u8) {
    self.place(cond as u32 | ((op as u32) << 24));
    self.commit();

    self.patch_label(label, Kind::IMM19);

    if self.log_text {
      self.log_c_char_label(name, *label);
    }
  }
}
