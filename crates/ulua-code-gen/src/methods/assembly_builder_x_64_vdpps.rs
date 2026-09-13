use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64,
};

impl AssemblyBuilderX64 {
  pub fn vdpps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64, mask: u8) {
    self.place_avx_imm8("vdpps", dst, src1, src2, mask, AvxOpEncoding::VDPPS);
  }
}
