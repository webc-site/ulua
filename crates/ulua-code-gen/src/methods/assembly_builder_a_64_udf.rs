use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn udf(&mut self) {
    self.place_0("udf", 0);
  }
}
