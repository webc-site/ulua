//! Test fixture: faithful port of `compileTypeTable` (tests/Compiler.test.cpp).
use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
pub fn compile_type_table(source: &str) -> String {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
  let options = CompileOptions {
    vector_type: c"Vector3".as_ptr(),
    type_info_level: 1,
    ..Default::default()
  };
  let source = String::from(source);
  let parse_options = ParseOptions::default();
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &source,
    &options,
    &parse_options,
  );
  bcb.dump_type_info()
}
