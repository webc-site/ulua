use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn frintp(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == src.kind());
    debug_assert!(dst.kind() == KindA64::D || dst.kind() == KindA64::S || dst.kind() == KindA64::Q);

    if dst.kind() == KindA64::Q {
      self.place_r_1("frintp", dst, src, 0b01_0011_1010_1000_0110_0010);
    } else if dst.kind() == KindA64::D {
      self.place_r_1("frintp", dst, src, 0b00_0111_1001_1001_0011_0000);
    } else {
      self.place_r_1("frintp", dst, src, 0b00_0111_1000_1001_0011_0000);
    }
  }
}
