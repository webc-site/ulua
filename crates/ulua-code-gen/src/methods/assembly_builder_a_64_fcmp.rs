use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fcmp(&mut self, src1: RegisterA64, src2: RegisterA64) {
    debug_assert!(src1.kind() == src2.kind());
    debug_assert!(src1.kind() == KindA64::D || src1.kind() == KindA64::S);

    if src1.kind() == KindA64::D {
      self.assembly_builder_a_64_place_fcmp("fcmp", src1, src2, 0b1111_0011, 0b00);
    } else {
      self.assembly_builder_a_64_place_fcmp("fcmp", src1, src2, 0b1111_0001, 0b00);
    }
  }
}
