//! Test fixture: faithful port of `compileFunction0` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::functions::compile_dumped_bcb::compile_dumped_bcb;
pub fn compile_function_0(source: &str) -> String {
  let flags = BytecodeBuilder::DUMP_CODE;
  let bcb = compile_dumped_bcb(source, &CompileOptions::default(), flags, None);
  bcb.dump_function(0)
}
