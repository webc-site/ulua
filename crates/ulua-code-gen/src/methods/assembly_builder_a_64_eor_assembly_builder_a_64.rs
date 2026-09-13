use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn eor_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.place_sr_3("eor", dst, src1, src2, 0b1001010, shift, 0);
  }
}
