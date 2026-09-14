use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

/// Returns an operand for the integer part of a TValue in a Luau register.
///
/// C++: dword[rBase + ri * sizeof(TValue) + offsetof(TValue, value)]
///
#[inline]
pub fn luau_reg_value_int(ri: i32) -> OperandX64 {
  OperandX64::mem(
    SizeX64::Dword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R14,
    ri * 16,
  )
}
