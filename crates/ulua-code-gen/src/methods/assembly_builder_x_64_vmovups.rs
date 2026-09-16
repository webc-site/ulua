use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vmovups(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
      "vmovups", dst, src,
      // C++: placeAvx("vmovups", dst, src, 0x10, 0x11, false, AVX_0F, AVX_NP);
      // prefix is AVX_NP (0b00), not 0x10.
      0x10, 0x11, false, 0x0F, 0x00,
    );
  }
}
