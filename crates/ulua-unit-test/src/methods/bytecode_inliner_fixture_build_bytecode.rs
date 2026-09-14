use alloc::vec::Vec;

use ulua_bytecode::{
  functions::from_function_bytecode::from_function_bytecode,
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;

impl BytecodeInlinerFixture {
  pub fn build_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<(CompTimeBcFunction, CompTimeBcFunction)> {
    let bytecode = self.get_function_bytecode(src, optimization_level);

    if let Some(bytecode) = bytecode {
      self.strings = bytecode.string_table.clone();
      let mut table: Vec<&[u8]> = Vec::new();
      for s in self.strings.iter() {
        table.push(s.as_bytes());
      }

      let inlinee = from_function_bytecode(bytecode.inlinee_bytecode, &mut table);
      LUAU_ASSERT!(inlinee.is_some());
      LUAU_ASSERT!(inlinee.as_ref().unwrap().debugname == "inlinee");

      let caller = from_function_bytecode(bytecode.caller_bytecode, &mut table);
      LUAU_ASSERT!(caller.is_some());
      LUAU_ASSERT!(caller.as_ref().unwrap().debugname == "caller");

      Some((inlinee.unwrap(), caller.unwrap()))
    } else {
      None
    }
  }
}
