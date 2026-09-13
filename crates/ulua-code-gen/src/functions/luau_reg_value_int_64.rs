use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

/// Returns an operand for the 64-bit integer part of a TValue in a Luau register.
///
/// C++: qword[rBase + ri * sizeof(TValue) + offsetof(TValue, value.l)]
///
#[inline]
pub fn luau_reg_value_int_64(ri: i32) -> OperandX64 {
  // TValue size is 16 bytes in Luau VM (Value value + int extra[2] + int tt)
  let tvalue_size = size_of::<TValue>() as i32;
  // offsetof(TValue, value.l) is 8 (Value.l is the 64-bit integer member)
  let value_l_offset = 8;

  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * tvalue_size + value_l_offset,
  )
}
