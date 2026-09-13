use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn place_fmov(&mut self, name: &str, dst: RegisterA64, src: f64, op: u32) {
    if self.log_text {
      self.log_c_char_register_a_64_f64(name, dst, src);
    }

    self.place(dst.index() as u32 | (op << 5));
    self.commit();
  }
}
