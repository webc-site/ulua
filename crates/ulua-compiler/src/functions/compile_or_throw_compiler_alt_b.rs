use std::panic::panic_any;

use ulua_ast::records::{parse_errors::ParseErrors, parse_options::ParseOptions};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

use crate::{
  functions::{
    compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
    parse_pinned::parse_pinned,
  },
  records::compile_options::CompileOptions,
};

pub fn compile_or_throw_bytecode_builder_string_compile_options_parse_options<B>(
  bytecode: &mut BytecodeBuilder,
  source: &B,
  options: &CompileOptions,
  parse_options: &ParseOptions,
) where
  B: AsRef<[u8]> + ?Sized,
{
  let (_allocator, mut names, result) = parse_pinned(source, parse_options);

  if !result.errors.is_empty() {
    // Faithful to C++ `throw ParseErrors(result.errors)`: panic with the
    // ParseErrors payload (not a bare "ParseErrors" string) so callers that
    // catch the unwind can downcast it and read the real message via Display
    // (`ParseErrors::what()` = the first error's message for a single error).
    panic_any(ParseErrors::new(result.errors.clone()));
  }

  compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
    bytecode, &result, &mut names, options,
  );
}
