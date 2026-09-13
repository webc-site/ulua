use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn blr(&mut self, src: RegisterA64) {
    self.place_br("blr", src, 0b11_0101_1000_1111_1100_0000);
  }
}
