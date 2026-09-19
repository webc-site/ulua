//! Source: `tests/Compiler.test.cpp`

use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
pub fn compile_with_remarks(source: &str) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_SOURCE | BytecodeBuilder::DUMP_REMARKS);
  bcb.set_dump_source(source);

  let options = CompileOptions {
    optimization_level: 2,
    ..Default::default()
  };
  let owned = String::from(source);
  let parse_options = ParseOptions::default();
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &owned,
    &options,
    &parse_options,
  );

  bcb.dump_source_remarks()
}
