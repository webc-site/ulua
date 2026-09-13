use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vmaxss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vmaxss",
      dst,
      src1,
      src2,
      0x5f,
      false,
      0b0000_1111u8,
      0b1111_0011u8,
    );
  }
}
