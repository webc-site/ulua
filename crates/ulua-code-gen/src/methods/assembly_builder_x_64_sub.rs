use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, binary_op_encoding::BinaryOpEncoding,
  operand_x_64::OperandX64,
};

impl AssemblyBuilderX64 {
  pub fn sub(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("sub", lhs, rhs, BinaryOpEncoding::SUB);
  }
}
