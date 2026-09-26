use alloc::vec::Vec;

use crate::records::bytecode_reg_type_info::BytecodeRegTypeInfo;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BytecodeTypeInfo {
  pub argument_types: Vec<u8>,
  pub reg_types: Vec<BytecodeRegTypeInfo>,
  pub upvalue_types: Vec<u8>,

  /// 各寄存器在 reg_types 中的偏移
  /// 末尾多出一个元素记录 vector 长度，便于按 arr[Rn], arr[Rn + 1] 做范围访问
  pub reg_type_offsets: Vec<u32>,
}
