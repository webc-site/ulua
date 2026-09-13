use ulua_common::enums::luau_bytecode_type;
pub const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;

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
