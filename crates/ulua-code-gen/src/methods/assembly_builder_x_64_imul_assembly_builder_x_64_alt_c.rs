use crate::records::{assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64};

impl AssemblyBuilderX64 {
  pub fn imul_operand_x_64_operand_x_64_i32(&mut self, dst: OperandX64, lhs: OperandX64, rhs: i32) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64_operand_x_64(
        "imul",
        dst,
        lhs,
        OperandX64::operand_x_64_i32(rhs),
      );
    }

    self.place_rex_register_x_64_operand_x_64(dst.base, lhs);

    if (rhs as i8) as i32 == rhs {
      self.place(0x6b);
      self.place_reg_and_mod_reg_mem(dst, lhs, 1);
      self.place_imm_8(rhs);
    } else {
      self.place(0x69);
      self.place_reg_and_mod_reg_mem(dst, lhs, 4);
      self.place_imm_32(rhs);
    }

    self.commit();
  }
}
