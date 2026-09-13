use alloc::vec::Vec;

use ulua_bytecode::{
  functions::from_function_bytecode::from_function_bytecode,
  type_aliases::comp_time_bc_function::CompTimeBcFunction,
};

use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;

impl BytecodeCompilerFixture {
  pub fn build_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<CompTimeBcFunction> {
    let bytecode = self.get_function_bytecode(src, optimization_level);

    if let Some((function_bytecode, strings)) = bytecode {
      self.strings = strings;
      let mut table: Vec<&[u8]> = Vec::new();
      for s in self.strings.iter() {
        table.push(s.as_bytes());
      }

      from_function_bytecode(function_bytecode, &mut table)
    } else {
      None
    }
  }
}
