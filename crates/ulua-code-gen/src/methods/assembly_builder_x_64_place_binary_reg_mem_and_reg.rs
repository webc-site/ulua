use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn place_binary_reg_mem_and_reg(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    code8: u8,
    code: u8,
  ) {
    // In two operand instructions, first operand is always a register, but data flow direction is reversed
    self.place_binary_reg_and_reg_mem(rhs, lhs, code8, code);
  }
}
