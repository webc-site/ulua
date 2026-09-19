use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn cdq(&mut self) {
    if self.log_text {
      self.log_c_char("cdq");
    }
    self.place(0x99);
    self.commit();
  }
}
