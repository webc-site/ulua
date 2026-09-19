use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn place_0(&mut self, name: &str, op: u32) {
    if self.log_text {
      self.log_c_char(name);
    }

    self.place(op);
    self.commit();
  }
}
