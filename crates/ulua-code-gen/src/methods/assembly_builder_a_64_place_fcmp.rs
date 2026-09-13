use crate::records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64};

impl AssemblyBuilderA64 {
  pub fn assembly_builder_a_64_place_fcmp(
    &mut self,
    name: &str,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    opc: u8,
  ) {
    if self.log_text {
      if opc != 0 {
        // C++: log(name, src1, 0) -> reg + "#0" immediate (shift defaults to 0)
        self.log_c_char_register_a_64_i32_i32(name, src1, 0, 0);
      } else {
        self.log_c_char_register_a_64_register_a_64(name, src1, src2);
      }
    }

    assert!(src1.kind() == src2.kind());

    self.place(
      ((opc as u32) << 3)
        | ((src1.index() as u32) << 5)
        | ((0b1000u32) << 10)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21),
    );
    self.commit();
  }
}
