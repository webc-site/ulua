use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn lsr_register_a_64_register_a_64_register_a_64(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
  ) {
    self.place_r_3("lsr", dst, src1, src2, 0b11010110, 0b001001);
  }
}
