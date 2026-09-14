use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn mvn_(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.place_sr_2("mvn", dst, src, 0b01_01010, 0b1);
  }
}
