use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub fn operator_add_register_x_64_register_x_64(
  base: RegisterX64,
  index: RegisterX64,
) -> OperandX64 {
  CODEGEN_ASSERT!(index.index() != 4, "sp cannot be used as index");
  CODEGEN_ASSERT!(base.size() == index.size());

  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::None,
    index,
    1,
    base,
    0,
  )
}
