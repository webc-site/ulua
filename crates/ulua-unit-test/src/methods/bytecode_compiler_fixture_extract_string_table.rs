use alloc::{string::String, vec::Vec};

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

use crate::{
  functions::extract_string_table::extract_string_table as extract_table,
  records::bytecode_compiler_fixture::BytecodeCompilerFixture,
};

impl BytecodeCompilerFixture {
  pub fn extract_string_table(&mut self, bcb: &BytecodeBuilder) -> Vec<String> {
    // 复用共享提取逻辑，避免与 inliner fixture 重复实现
    extract_table(bcb.get_bytecode().as_bytes())
  }
}
