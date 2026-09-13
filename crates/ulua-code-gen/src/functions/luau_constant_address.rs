use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::records::{
  operand_x_64::{ADDR, OperandX64},
  register_x_64::RegisterX64,
};

pub fn luau_constant_address(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;

  ADDR.operand_x_64_operator_index(RegisterX64::R12 + ki * tvalue_size)
}
