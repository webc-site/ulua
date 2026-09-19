use ulua_common::enums::luau_bytecode_type::{LBC_TYPE_NIL, LuauBytecodeType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedLocal {
  pub(crate) r#type: LuauBytecodeType,
  pub(crate) reg: u8,
  pub(crate) startpc: u32,
  pub(crate) endpc: u32,
}

impl Default for TypedLocal {
  fn default() -> Self {
    Self {
      r#type: LBC_TYPE_NIL,
      reg: 0,
      startpc: 0,
      endpc: 0,
    }
  }
}
