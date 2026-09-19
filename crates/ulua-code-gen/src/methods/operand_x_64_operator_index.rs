use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

impl OperandX64 {
  pub fn operand_x_64_operator_index(&self, mut addr: OperandX64) -> OperandX64 {
    CODEGEN_ASSERT!(self.cat == CategoryX64::Mem);
    CODEGEN_ASSERT!(
      self.index == RegisterX64::NOREG
        && self.scale == 1
        && self.base == RegisterX64::NOREG
        && self.imm == 0
    );
    CODEGEN_ASSERT!(addr.mem_size == SizeX64::None);

    addr.cat = CategoryX64::Mem;
    addr.mem_size = self.mem_size;
    addr
  }
}
