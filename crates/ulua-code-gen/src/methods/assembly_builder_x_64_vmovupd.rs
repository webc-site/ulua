use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vmovupd(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
      "vmovupd", dst, src, 0x10, 0x11, false, 0x0F, 0x66,
    );
  }
}
