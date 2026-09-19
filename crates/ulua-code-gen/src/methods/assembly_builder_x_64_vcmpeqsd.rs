use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, avx_op_encoding::AvxOpEncoding,
  operand_x_64::OperandX64,
};

impl AssemblyBuilderX64 {
  pub fn vcmpeqsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    // C++: placeAvx("vcmpeqsd", dst, src1, src2, 0x00, 0xc2, false, AVX_0F, AVX_F2);
    // imm8-carrying overload: imm8=0x00, code(opcode)=0xc2.
    self.place_avx_imm8("vcmpeqsd", dst, src1, src2, 0x00, AvxOpEncoding::VCMP);
  }
}
