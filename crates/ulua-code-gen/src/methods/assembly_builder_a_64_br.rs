use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn br(&mut self, src: RegisterA64) {
    self.place_br("br", src, 0b11_0101_1000_0111_1100_0000);
  }
}
