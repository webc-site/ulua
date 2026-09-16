use crate::{
  enums::size_x_64::SizeX64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

pub fn operator_deref(reg: RegisterX64, scale: u8) -> OperandX64 {
  if scale == 1 {
    return OperandX64::reg(reg);
  }

  CODEGEN_ASSERT!(scale == 1 || scale == 2 || scale == 4 || scale == 8);
  CODEGEN_ASSERT!(reg.index() != 0b100);

  OperandX64::mem(SizeX64::None, reg, scale, RegisterX64::NOREG, 0)
}
