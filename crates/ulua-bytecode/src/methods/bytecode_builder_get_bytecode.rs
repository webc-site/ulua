use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// finalize 后的字节码 blob 原始字节。
  pub fn get_bytecode(&self) -> &[u8] {
    LUAU_ASSERT!(!self.bytecode.is_empty()); // did you forget to call finalize?
    &self.bytecode
  }
}
