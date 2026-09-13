use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn rev(&mut self, dst: RegisterA64, src: RegisterA64) {
    CODEGEN_ASSERT!(dst.kind() == KindA64::W || dst.kind() == KindA64::X);
    CODEGEN_ASSERT!(dst.kind() == src.kind());

    self.place_r_1(
      "rev",
      dst,
      src,
      0b1_0110_1011_0000_0000_0010 | (dst.kind() == KindA64::X) as u32,
    );
  }
}
