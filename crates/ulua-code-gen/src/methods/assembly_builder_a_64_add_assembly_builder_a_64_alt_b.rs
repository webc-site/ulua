use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn add_register_a_64_register_a_64_u16(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u16,
  ) {
    self.place_i12("add", dst, src1, src2 as i32, 0b00_10001);
  }
}
