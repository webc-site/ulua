use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn fcsel(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
  ) {
    CODEGEN_ASSERT!(dst.kind() == src1.kind() && src1.kind() == src2.kind());
    CODEGEN_ASSERT!(dst.kind() == KindA64::D || dst.kind() == KindA64::S);

    if src1.kind() == KindA64::D {
      self.place_cs("fcsel", dst, src1, src2, cond, 0b1111_0011, 0b11, 0);
    } else {
      self.place_cs("fcsel", dst, src1, src2, cond, 0b1111_0001, 0b11, 0);
    }
  }
}
