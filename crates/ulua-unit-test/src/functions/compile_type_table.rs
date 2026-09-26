//! Test fixture: faithful port of `compileTypeTable` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::{functions::compile_dumped_bcb::compile_dumped_bcb, macros::cstr_names::NAME_VECTOR3};
pub fn compile_type_table(source: &str) -> String {
  let options = CompileOptions {
    vector_type: NAME_VECTOR3.as_ptr().cast(),
    type_info_level: 1,
    ..Default::default()
  };
  let bcb = compile_dumped_bcb(source, &options, BytecodeBuilder::DUMP_CODE, None);
  bcb.dump_type_info()
}
