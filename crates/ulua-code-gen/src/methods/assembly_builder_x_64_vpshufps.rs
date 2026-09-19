use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn vpshufps(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, shuffle: u8) {
    self.place_avx_imm8(
      "vpshufps",
      OperandX64::reg(dst),
      OperandX64::reg(src1),
      src2,
      shuffle,
      AvxOpEncoding::VPSHUTFPS,
    );
  }
}
