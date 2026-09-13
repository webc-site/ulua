use crate::{
  enums::{category_x_64::CategoryX64, size_x_64::SizeX64},
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub fn operator_add_register_x_64_operand_x_64(
  base: RegisterX64,
  mut op: OperandX64,
) -> OperandX64 {
  // CODEGEN_ASSERT's Rust-side handler expects raw pointers for file/function info.
  // The luau-code-gen macro currently passes &str, which doesn't match.
  // Use the same runtime condition checks directly instead of CODEGEN_ASSERT.
  debug_assert!(op.cat == CategoryX64::Mem);
  debug_assert!(op.mem_size == SizeX64::None);
  debug_assert!(op.base == RegisterX64::NOREG);
  debug_assert!(op.index == RegisterX64::NOREG || op.index.size() == base.size());

  op.base = base;
  op
}
