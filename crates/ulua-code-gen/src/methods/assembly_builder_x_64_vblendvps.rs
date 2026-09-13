use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn vblendvps(
    &mut self,
    _dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    // bits [7:4] of imm8 are used to select register for operand 4
    self.place_avx_imm8(
      "vblendvps",
      OperandX64::reg(RegisterX64::NOREG),
      src1.into(),
      src2,
      mask.index() << 4,
      AvxOpEncoding::VBLENDVPS,
    );
  }
}
