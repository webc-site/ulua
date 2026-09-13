use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, binary_op_encoding::BinaryOpEncoding,
  operand_x_64::OperandX64,
};

impl AssemblyBuilderX64 {
  pub fn and_(&mut self, lhs: OperandX64, rhs: OperandX64) {
    // C++ `placeBinary("and", lhs, rhs, 0x80, 0x81, 0x83, 0x20, 0x21, 0x22, 0x23, 4)`.
    // The original port used the imm-only `place_binary_reg_mem_and_imm` (which
    // can't encode reg/reg or reg/mem forms) AND passed `0x20` where the ModRM
    // opcode-extension digit must be 4 (AND) — yielding the ADD (/0) encoding.
    self.place_binary("and", lhs, rhs, BinaryOpEncoding::AND);
  }
}
