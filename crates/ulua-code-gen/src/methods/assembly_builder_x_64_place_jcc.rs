use crate::{
  macros::op_plus_cc::op_plus_cc,
  records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label},
};

impl AssemblyBuilderX64 {
  pub fn place_jcc(&mut self, name: &str, label: &mut Label, cc: u8) {
    self.place(0x0f);
    self.place(op_plus_cc(0x80, cc));
    self.place_label(label);

    if self.log_text {
      self.log_c_char_label(name, *label);
    }

    self.commit();
  }
}
