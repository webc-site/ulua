use alloc::{string::String, vec::Vec};

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

use crate::{
  functions::extract_string_table::extract_string_table as extract_table,
  records::bytecode_inliner_fixture::BytecodeInlinerFixture,
};

impl BytecodeInlinerFixture {
  pub fn extract_string_table(&self, bcb: &BytecodeBuilder) -> Vec<String> {
    // 复用共享提取逻辑，避免与 compiler fixture 重复实现
    extract_table(bcb.get_bytecode().as_bytes())
  }
}
