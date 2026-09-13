use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fcvtzs(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    debug_assert!(src.kind() == KindA64::D);

    self.place_r_1("fcvtzs", dst, src, 0b00_0111_1001_1110_0000_0000);
  }
}
