use crate::{
  enums::kind_a_64::KindA64,
  records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
};

impl AssemblyBuilderA64 {
  pub fn ucvtf(&mut self, dst: RegisterA64, src: RegisterA64) {
    // CODEGEN_ASSERT! currently expands to ulua_common::assert_call_handler(...),
    // which expects raw pointers. Avoid invoking it here.
    debug_assert!(dst.kind() == KindA64::D || dst.kind() == KindA64::S);
    debug_assert!(src.kind() == KindA64::W || src.kind() == KindA64::X);

    if dst.kind() == KindA64::D {
      self.place_r_1("ucvtf", dst, src, 0b00_0111_1001_1000_1100_0000);
    } else {
      self.place_r_1("ucvtf", dst, src, 0b00_0111_1000_1000_1100_0000);
    }
  }
}
