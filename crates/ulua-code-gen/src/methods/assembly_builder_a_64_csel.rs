use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn csel(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
  ) {
    debug_assert!(dst.kind() == KindA64::X || dst.kind() == KindA64::W);

    self.place_cs("csel", dst, src1, src2, cond, 0b11010100, 0b00, 0);
  }
}
