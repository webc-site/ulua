use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn log_c_char_operand_x_64_operand_x_64_operand_x_64_operand_x_64(
    &mut self,
    opcode: &str,
    op1: OperandX64,
    op2: OperandX64,
    op3: OperandX64,
    op4: OperandX64,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand_x_64(op1);
    self.text.push(',');
    self.log_operand_x_64(op2);
    self.text.push(',');
    self.log_operand_x_64(op3);
    self.text.push(',');
    self.log_operand_x_64(op4);
    self.text.push('\n');
  }
}
