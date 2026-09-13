use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::operand_x_64::OperandX64,
};

pub fn operator_add_operand_x_64_i32(mut op: OperandX64, disp: i32) -> OperandX64 {
  debug_assert!(op.cat == CategoryX64::Mem);
  debug_assert!(op.mem_size == SizeX64::None);

  op.imm += disp;
  op
}
