//! Test fixture: faithful port of `compileTypeTable` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::functions::compile_dumped_bcb::compile_dumped_bcb;
pub fn compile_type_table(source: &str) -> String {
  let options = CompileOptions::new()
    .with_vector_type(Some("Vector3"))
    .with_type_info_level(1);
  let bcb = compile_dumped_bcb(source, &options, BytecodeBuilder::DUMP_CODE, None);
  bcb.dump_type_info()
}
