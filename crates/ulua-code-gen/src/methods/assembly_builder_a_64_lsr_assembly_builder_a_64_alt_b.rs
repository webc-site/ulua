use crate::{
  enums::kind_a_64::KindA64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn lsr_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u8,
  ) {
    let size = if dst.kind() == KindA64::X { 64 } else { 32 };
    CODEGEN_ASSERT!(src2 < size as u8);

    self.place_bfm(
      "lsr",
      dst,
      src1,
      src2 as i32,
      0b10_100110,
      src2 as i32,
      size - 1,
    );
  }
}
