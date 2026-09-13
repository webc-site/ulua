use crate::{
  enums::category_x_64::CategoryX64,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, binary_op_encoding::BinaryOpEncoding,
    operand_x_64::OperandX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn place_binary(
    &mut self,
    name: &str,
    lhs: OperandX64,
    rhs: OperandX64,
    enc: BinaryOpEncoding,
  ) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64(name, lhs, rhs);
    }

    if (lhs.cat == CategoryX64::Reg || lhs.cat == CategoryX64::Mem) && rhs.cat == CategoryX64::Imm {
      self.place_binary_reg_mem_and_imm(
        lhs,
        rhs,
        enc.codeimm8,
        enc.codeimm,
        enc.codeimm_imm8,
        enc.opreg,
      );
    } else if lhs.cat == CategoryX64::Reg
      && (rhs.cat == CategoryX64::Reg || rhs.cat == CategoryX64::Mem)
    {
      self.place_binary_reg_and_reg_mem(lhs, rhs, enc.code8, enc.code);
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Reg {
      self.place_binary_reg_mem_and_reg(lhs, rhs, enc.code8rev, enc.coderev);
    } else {
      // Avoid CODEGEN_ASSERT! due to assert_call_handler signature mismatch in this crate.
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }
}
