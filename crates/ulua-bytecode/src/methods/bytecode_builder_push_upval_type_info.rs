use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::records::{bytecode_builder::BytecodeBuilder, typed_upval::TypedUpval};

impl BytecodeBuilder {
  pub fn push_upval_type_info(&mut self, r#type: LuauBytecodeType) {
    let upval = TypedUpval { r#type };
    self.typed_upvals.push(upval);
  }
}
