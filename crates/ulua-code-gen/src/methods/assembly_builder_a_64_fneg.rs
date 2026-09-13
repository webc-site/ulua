use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fneg(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::D {
      debug_assert!(src.kind() == KindA64::D);

      self.place_r_1("fneg", dst, src, 0b00_0111_1001_1000_0101_0000);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src.kind() == KindA64::S);

      self.place_r_1("fneg", dst, src, 0b00_0111_1000_1000_0101_0000);
    } else {
      debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::Q);

      self.place_r_1("fneg", dst, src, 0b01_1011_1010_1000_0011_1110);
    }
  }
}
