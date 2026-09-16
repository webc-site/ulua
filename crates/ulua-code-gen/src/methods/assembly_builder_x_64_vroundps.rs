use crate::{
  enums::rounding_mode_x_64::RoundingModeX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn vroundps(&mut self, dst: OperandX64, src: OperandX64, rounding_mode: RoundingModeX64) {
    // 'placeAvx' wrapper doesn't have an overload for this archetype (opcode r/m, reg, imm8)
    if self.log_text {
      // C++: log("vroundps", dst, src, uint8_t(roundingMode) | kRoundingPrecisionInexact)
      self.log_c_char_operand_x_64_operand_x_64_operand_x_64(
        "vroundps",
        dst,
        src,
        OperandX64::from((rounding_mode as u8 | 0x08) as i32),
      );
    }

    // C++: placeVex(dst, noreg, src, false, AVX_0F3A, AVX_66); place(0x08);
    //      placeRegAndModRegMem(dst, src, 1);
    //      placeImm8(roundingMode | kRoundingPrecisionInexact);
    // mode is AVX_0F3A (0x3A -> 0b00011), and the imm8 precision flag is
    // kRoundingPrecisionInexact (0b1000 = 0x08), not 0x04.
    self.place_vex(
      dst,
      OperandX64::reg(RegisterX64::NOREG),
      src,
      false,
      0x3A,
      0x66,
    );
    self.place(0x08);
    self.place_reg_and_mod_reg_mem(dst, src, 1);
    self.place_imm_8((rounding_mode as u8 | 0x08) as i32);

    self.commit();
  }
}
