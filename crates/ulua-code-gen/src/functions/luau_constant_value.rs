use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

/// Returns an operand for the value field of a constant TValue in the constant array.
///
/// C++: qword[rConstants + ki * sizeof(TValue) + offsetof(TValue, value)]
///
/// In Luau x64 JIT:
/// - rConstants is a base pointer to the constant array
/// - sizeof(TValue) is 16 (2^kTValueSizeLog2, where kTValueSizeLog2 = 4)
/// - offsetof(TValue, value) is 0 (Value is the first member of the union)
/// - The operand uses qword size for the full TValue value field access
#[inline]
pub fn luau_constant_value(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let value_offset = 0;

  // NOTE: rConstants is represented as a memory base register operand, not as a RegisterX64::rconstants associated constant.
  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::Qword,
    RegisterX64::NOREG,
    0,
    RegisterX64::RBP,
    ki * tvalue_size + value_offset,
  )
}
