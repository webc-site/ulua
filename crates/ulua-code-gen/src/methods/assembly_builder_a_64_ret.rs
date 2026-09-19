use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn ret(&mut self) {
    self.place_0("ret", 0b1101_0110_0101_1111_0000_0011_1100_0000);
  }
}
