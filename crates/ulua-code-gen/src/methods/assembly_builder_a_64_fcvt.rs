use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn fcvt(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::S && src.kind() == KindA64::D {
      self.place_r_1("fcvt", dst, src, 0b111_1001_1000_1001_0000);
    } else if dst.kind() == KindA64::D && src.kind() == KindA64::S {
      self.place_r_1("fcvt", dst, src, 0b111_1000_1000_1011_0000);
    } else {
      ulua_common::LUAU_ASSERT!(false);
    }
  }
}
