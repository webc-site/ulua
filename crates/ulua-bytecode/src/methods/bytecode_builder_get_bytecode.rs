use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_bytecode(&self) -> &String {
    LUAU_ASSERT!(!self.bytecode.is_empty()); // did you forget to call finalize?
    &self.bytecode
  }
}
