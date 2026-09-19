use ulua_common::enums::luau_bytecode_type;
pub const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;

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
