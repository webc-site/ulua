use crate::{
  enums::rounding_mode_x_64::RoundingModeX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64},
};

impl AssemblyBuilderX64 {
  pub fn vroundss(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    rounding_mode: RoundingModeX64,
  ) {
    self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
      "vroundss",
      dst,
      src1,
      src2,
      rounding_mode as u8 | 0x04,
      false,
      0x0a,
      0x66,
    );
  }
}
