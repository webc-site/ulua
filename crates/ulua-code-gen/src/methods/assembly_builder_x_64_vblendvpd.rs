use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn vblendvpd(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    // bits [7:4] of imm8 are used to select register for operand 4
    // C++: placeAvx("vblendvpd", dst, src1, src2, mask.index << 4, 0x4b,
    //               false, AVX_0F3A, AVX_66);
    // This needs the imm8-carrying overload: imm8 = mask.index << 4,
    // code(opcode) = 0x4b, mode = AVX_0F3A (0x3A -> 0b00011).
    self.place_avx_imm8(
      "vblendvpd",
      OperandX64::reg(dst),
      OperandX64::reg(src1),
      src2,
      mask.index() << 4,
      AvxOpEncoding::VBLENDVPD,
    );
  }
}
