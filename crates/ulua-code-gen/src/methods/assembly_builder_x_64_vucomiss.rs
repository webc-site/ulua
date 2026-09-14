use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vucomiss(&mut self, src1: OperandX64, src2: OperandX64) {
    // C++: placeAvx("vucomiss", src1, src2, 0x2e, false, AVX_0F, AVX_NP);
    // 2-operand overload — no vvvv source (VEX.vvvv = 1111).
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vucomiss", src1, src2, 0x2e, false, 0x0F, // AVX_0F
      0x00, // AVX_NP
    );
  }
}
