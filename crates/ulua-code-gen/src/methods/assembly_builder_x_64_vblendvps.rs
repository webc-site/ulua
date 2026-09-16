use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl AssemblyBuilderX64 {
  pub fn vblendvps(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    // bits [7:4] of imm8 are used to select register for operand 4
    // cpp:1045 传真实 dst（此前误传 NOREG 占位，日志打印时越界）
    self.place_avx_imm8(
      "vblendvps",
      OperandX64::reg(dst),
      src1.into(),
      src2,
      mask.index() << 4,
      AvxOpEncoding::VBLENDVPS,
    );
  }
}
