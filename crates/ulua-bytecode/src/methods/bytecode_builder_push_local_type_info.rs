use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::records::{bytecode_builder::BytecodeBuilder, typed_local_bytecode_builder::TypedLocal};

impl BytecodeBuilder {
  pub fn push_local_type_info(
    &mut self,
    r#type: LuauBytecodeType,
    reg: u8,
    startpc: u32,
    endpc: u32,
  ) {
    let local = TypedLocal {
      r#type,
      reg,
      startpc,
      endpc,
    };

    self.typed_locals.push(local);
  }
}
