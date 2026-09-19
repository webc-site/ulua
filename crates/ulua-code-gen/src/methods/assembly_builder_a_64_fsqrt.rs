use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fsqrt(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == src.kind());
    debug_assert!(dst.kind() == KindA64::D || dst.kind() == KindA64::S);

    if dst.kind() == KindA64::D {
      self.place_r_1("fsqrt", dst, src, 0b00_0111_1001_1000_0111_0000);
    } else {
      self.place_r_1("fsqrt", dst, src, 0b00_0111_1000_1000_0111_0000);
    }
  }
}
