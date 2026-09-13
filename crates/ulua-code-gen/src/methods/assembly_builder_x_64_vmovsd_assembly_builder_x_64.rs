use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

const AVX_0F: u8 = 0b0001;
const AVX_F2: u8 = 0b11;

impl AssemblyBuilderX64 {
  pub fn vmovsd_operand_x_64_operand_x_64(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_u8_u8_bool_u8_u8(
      "vmovsd", dst, src, 0x10, 0x11, false, AVX_0F, AVX_F2,
    );
  }
}
