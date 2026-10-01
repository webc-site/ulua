use alloc::vec::Vec;

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeBlock {
  /// 'start' 与 'finish' 定义属于本 block 的指令闭区间
  pub startpc: i32,
  pub finishpc: i32,
}

impl Default for BytecodeBlock {
  fn default() -> Self {
    Self {
      startpc: -1,
      finishpc: -1,
    }
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeMapping {
  pub ir_location: u32,
  pub asm_location: u32,
}

pub const LBC_TYPE_ANY: u8 = LuauBytecodeType::LBC_TYPE_ANY.0 as u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BytecodeRegTypeInfo {
  pub r#type: u8,
  pub reg: u8,
  pub startpc: i32,
  pub endpc: i32,
}

impl Default for BytecodeRegTypeInfo {
  fn default() -> Self {
    Self {
      r#type: LBC_TYPE_ANY,
      reg: 0,
      startpc: 0,
      endpc: 0,
    }
  }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BytecodeTypes {
  pub result: u8,
  pub a: u8,
  pub b: u8,
  pub c: u8,
}

impl Default for BytecodeTypes {
  fn default() -> Self {
    Self {
      result: LBC_TYPE_ANY,
      a: LBC_TYPE_ANY,
      b: LBC_TYPE_ANY,
      c: LBC_TYPE_ANY,
    }
  }
}
