use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
  pub fn nop(&mut self, bytes: u32) {
    let count = bytes / 4;
    for _ in 0..count {
      self.place_0("nop", 0b11010101000000110010000000011111u32);
    }
  }
}
