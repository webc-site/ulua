use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn ret(&mut self) {
    if self.log_text {
      self.log_c_char("ret");
    }

    self.place(0xc3);
    self.commit();
  }
}
