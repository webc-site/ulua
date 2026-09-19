use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn vpextrd(&mut self, dst: RegisterX64, src: RegisterX64, offset: u8) {
    // 'placeAvx' wrapper doesn't have an overload for this archetype (opcode r/m, reg, imm8)
    if self.log_text {
      // C++: log("vpextrd", dst, src, offset)
      self.log_c_char_operand_x_64_operand_x_64_operand_x_64(
        "vpextrd",
        dst.into(),
        src.into(),
        OperandX64::from(offset as i32),
      );
    }

    // C++: placeVex(src, noreg, dst, false, AVX_0F3A, AVX_66);
    // opcode map is AVX_0F3A (0x3A -> 0b00011), not 0x10.
    self.place_vex(
      src.into(),
      OperandX64::reg(RegisterX64::NOREG),
      dst.into(),
      false,
      0x3A,
      0x66,
    );
    self.place(0x16);
    self.place_reg_and_mod_reg_mem(src.into(), dst.into(), 1);
    self.place_imm_8(offset as i32);

    self.commit();
  }
}
