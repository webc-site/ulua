use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub fn operator_add_operand_x_64_register_x_64(op: OperandX64, base: RegisterX64) -> OperandX64 {
  debug_assert!(op.cat == CategoryX64::Mem);
  debug_assert!(op.mem_size == SizeX64::None);
  debug_assert!(op.base == RegisterX64::NOREG);
  debug_assert!(op.index == RegisterX64::NOREG || op.index.size() == base.size());

  let mut op = op;
  op.base = base;
  op
}
