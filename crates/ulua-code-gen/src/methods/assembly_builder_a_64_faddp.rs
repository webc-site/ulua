use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn faddp(&mut self, dst: RegisterA64, src: RegisterA64) {
    CODEGEN_ASSERT!(dst.kind() == KindA64::D || dst.kind() == KindA64::S);
    CODEGEN_ASSERT!(dst.kind() == src.kind());

    let is_d = if dst.kind() == KindA64::D { 1 } else { 0 };
    let op = 0b01_1111_1000_1100_0011_0110 | (is_d << 12);

    self.place_r_1("faddp", dst, src, op);
  }
}
