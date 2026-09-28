//! Source: `tests/Compiler.test.cpp`
use alloc::string::String;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::functions::compile_dumped_bcb::compile_dumped_bcb;
pub fn compile_with_remarks(source: &str) -> String {
  let options = CompileOptions {
    optimization_level: 2,
    ..Default::default()
  };
  let flags = BytecodeBuilder::DUMP_SOURCE | BytecodeBuilder::DUMP_REMARKS;
  let bcb = compile_dumped_bcb(source, &options, flags, Some(source));
  bcb.dump_source_remarks()
}
