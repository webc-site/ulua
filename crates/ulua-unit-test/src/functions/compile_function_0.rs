//! Test fixture: faithful port of `compileFunction0` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
pub fn compile_function_0(source: &str) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
  let source = String::from(source);
  let options = CompileOptions::default();
  let parse_options = ParseOptions::default();
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &source,
    &options,
    &parse_options,
  );
  bcb.dump_function(0)
}
