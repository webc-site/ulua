use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};
#[inline]
pub fn luau_constant_tag(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let tt_offset = core::mem::offset_of!(TValue, tt) as i32;

  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::Dword,
    RegisterX64::NOREG,
    0,
    RegisterX64::RBP,
    ki * tvalue_size + tt_offset,
  )
}
