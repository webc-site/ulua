use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

const AVX_0F: u8 = 0b0001;
const AVX_F3: u8 = 0b10;

impl AssemblyBuilderX64 {
  // C++ AssemblyBuilderX64::vmovss(OperandX64 dst, OperandX64 src1, OperandX64 src2)
  pub fn vmovss_operand_x_64_operand_x_64_operand_x_64(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
  ) {
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vmovss", dst, src1, src2, 0x10, false, AVX_0F, AVX_F3,
    );
  }
}
