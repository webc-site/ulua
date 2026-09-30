use alloc::vec::Vec;

use crate::{
  functions::parse_and_compile::parse_and_compile,
  records::bytecode_compiler_fixture::BytecodeCompilerFixture,
};

impl BytecodeCompilerFixture {
  pub fn get_function_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<(Vec<u8>, Vec<Vec<u8>>)> {
    let bcb = parse_and_compile(src, optimization_level)?;
    Some((bcb.get_function_data(0), self.extract_string_table(&bcb)))
  }
}
