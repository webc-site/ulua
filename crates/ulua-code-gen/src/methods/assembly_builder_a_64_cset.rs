use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};
impl AssemblyBuilderA64 {
  pub fn cset(&mut self, dst: RegisterA64, cond: ConditionA64) {
    CODEGEN_ASSERT!(dst.kind() == KindA64::X || dst.kind() == KindA64::W);

    let src = if dst.kind() == KindA64::X {
      RegisterA64::XZR
    } else {
      RegisterA64::WZR
    };

    self.place_cs("cset", dst, src, src, cond, 0b1101_0100, 0b01, 1);
  }
}
