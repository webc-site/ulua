use std::panic::panic_any;

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_errors::ParseErrors,
  parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;

use crate::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_options::CompileOptions,
};

pub fn compile_or_throw_bytecode_builder_string_compile_options_parse_options(
  bytecode: &mut BytecodeBuilder,
  source: &str,
  options: &CompileOptions,
  parse_options: &ParseOptions,
) {
  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    source,
    source.len(),
    &mut names,
    &mut allocator,
    parse_options.clone(),
  );

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
