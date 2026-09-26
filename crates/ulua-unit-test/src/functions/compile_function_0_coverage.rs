//! Test fixture: faithful port of `compileFunction0Coverage` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::functions::compile_dumped_bcb::compile_dumped_bcb;
pub fn compile_function_0_coverage(source: &str, level: i32) -> String {
  let options = CompileOptions {
    coverage_level: level,
    ..Default::default()
  };
  let flags = BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES;
  let bcb = compile_dumped_bcb(source, &options, flags, None);
  bcb.dump_function(0)
}
