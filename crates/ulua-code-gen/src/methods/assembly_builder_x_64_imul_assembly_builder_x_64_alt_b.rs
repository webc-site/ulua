use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn imul_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, rhs: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64("imul", lhs, rhs);
    }

    self.place_rex_register_x_64_operand_x_64(lhs.base, rhs);
    self.place(0x0f);
    self.place(0xaf);
    self.place_reg_and_mod_reg_mem(lhs, rhs, 0);
    self.commit();
  }
}
