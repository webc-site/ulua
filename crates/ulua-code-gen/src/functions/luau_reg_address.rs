use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

#[inline]
pub fn luau_reg_address(ri: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;

  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::None,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size,
  )
}
