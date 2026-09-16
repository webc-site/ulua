use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn vdivps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vdivps", dst, src1, src2, 0x5e, false, 0x0F, 0,
    );
  }
}
