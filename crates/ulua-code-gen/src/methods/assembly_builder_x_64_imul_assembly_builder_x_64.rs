use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn imul_operand_x_64(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("imul", op, 0xf6, 0xf7, 5);
  }
}
