use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn clz(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    debug_assert!(dst.kind() == src.kind());

    self.place_r_1("clz", dst, src, 0b1_0110_1011_0000_0000_0100);
  }
}
