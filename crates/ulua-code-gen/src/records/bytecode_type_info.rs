use alloc::vec::Vec;

use crate::records::bytecode_reg_type_info::BytecodeRegTypeInfo;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct BytecodeTypeInfo {
  pub argument_types: Vec<u8>,
  pub reg_types: Vec<BytecodeRegTypeInfo>,
  pub upvalue_types: Vec<u8>,

  /// Offsets into reg_types for each individual register
  /// One extra element at the end contains the vector size for easier arr[Rn], arr[Rn + 1] range access
  pub reg_type_offsets: Vec<u32>,
}
