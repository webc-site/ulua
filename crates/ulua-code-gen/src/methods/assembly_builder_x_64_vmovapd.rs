use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vmovapd(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
      "vmovapd", dst, src, 0x28, 0x29, false, 0x0F, 0x66,
    );
  }
}
