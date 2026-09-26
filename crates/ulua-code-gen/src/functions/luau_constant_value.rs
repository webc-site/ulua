use core::mem::size_of;

use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

/// 返回常量数组中 constant TValue 的 value 字段对应的操作数。
///
/// C++: qword[rConstants + ki * sizeof(TValue) + offsetof(TValue, value)]
///
/// Luau x64 JIT 中：
/// - rConstants 是常量数组的基址指针
/// - sizeof(TValue) 为 16（2^kTValueSizeLog2，kTValueSizeLog2 = 4）
/// - offsetof(TValue, value) 为 0（Value 是 union 的首个成员）
/// - 操作数用 qword 尺寸访问完整的 TValue value 字段
#[inline]
pub fn luau_constant_value(ki: i32) -> OperandX64 {
  let tvalue_size = size_of::<TValue>() as i32;
  let value_offset = 0;

  // cpp EmitCommonX64.h luauConstantValue: qword[rConstants + ki*sizeof(TValue) + offsetof(TValue,value)], rConstants=r12
  OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::R12,
    ki * tvalue_size + value_offset,
  )
}
