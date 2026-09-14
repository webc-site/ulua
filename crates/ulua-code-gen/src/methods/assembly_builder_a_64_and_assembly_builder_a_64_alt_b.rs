use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn and_register_a_64_register_a_64_u32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u32,
  ) {
    self.place_bm("and", dst, src1, src2, 0b00_100100);
  }
}
