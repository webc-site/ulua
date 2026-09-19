use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_x_64::AssemblyBuilderX64, operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

impl AssemblyBuilderX64 {
  pub fn lea_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, mut rhs: OperandX64) {
    if self.log_text {
      self.log_c_char_operand_x_64_operand_x_64("lea", lhs, rhs);
    }

    CODEGEN_ASSERT!(
      lhs.cat == CategoryX64::Reg && rhs.cat == CategoryX64::Mem && rhs.mem_size == SizeX64::None
    );
    CODEGEN_ASSERT!(rhs.base == RegisterX64::RIP || rhs.base.size() == lhs.base.size());
    CODEGEN_ASSERT!(rhs.index == RegisterX64::NOREG || rhs.index.size() == lhs.base.size());

    rhs.mem_size = lhs.base.size();

    self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8d, 0x8d);
  }
}
