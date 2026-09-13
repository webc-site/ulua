use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, binary_op_encoding::BinaryOpEncoding,
  operand_x_64::OperandX64,
};

impl AssemblyBuilderX64 {
  pub fn cmp(&mut self, lhs: OperandX64, rhs: OperandX64) {
    // C++ `placeBinary("cmp", lhs, rhs, 0x80, 0x81, 0x83, 0x38, 0x39, 0x3a, 0x3b, 7)`.
    // The original port used the imm-only `place_binary_reg_mem_and_imm`, which
    // DEBUGBREAKs on reg/reg or reg/mem comparisons (asserts rhs is an imm).
    self.place_binary("cmp", lhs, rhs, BinaryOpEncoding::CMP);
  }
}
