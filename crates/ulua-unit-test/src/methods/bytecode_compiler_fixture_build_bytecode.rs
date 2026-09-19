use alloc::vec::Vec;

use ulua_bytecode::{
  functions::from_function_bytecode::from_function_bytecode, records::bc_function::BcFunction,
};

use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;

impl BytecodeCompilerFixture {
  pub fn build_bytecode(&mut self, src: &str, optimization_level: i32) -> Option<BcFunction> {
    let (function_bytecode, strings) = self.get_function_bytecode(src, optimization_level)?;
    self.strings = strings;
    let table: Vec<&[u8]> = self.strings.iter().map(Vec::as_slice).collect();
    from_function_bytecode(&function_bytecode, &table)
  }
}
