use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn set_main_function(&mut self, fid: u32) {
    LUAU_ASSERT!(fid < self.functions.len() as u32);

    self.main_function = fid;
  }
}
