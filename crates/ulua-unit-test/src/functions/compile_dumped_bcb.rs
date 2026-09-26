//! 六件 compile 夹具（compile_function / compile_function_0 /
//! compile_function_0_constants / compile_function_0_coverage / compile_type_table /
//! compile_with_remarks）共用前段：建 bcb → 设 dump flags →（可选）预置 dump
//! source → compileOrThrow。镜像 tests/Compiler.test.cpp 各夹具的
//! `compileOrThrow(BytecodeBuilder{}, source)` 样板。
//! 镜像 tests/Compiler.test.cpp 各夹具共用的
//! `compileOrThrow(BytecodeBuilder{}, source)` 样板。
use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

/// 以默认 parse options 编译 `source` 进配好 `dump_flags` 的 bcb。
/// `dump_source` 须在编译前预置：`DUMP_SOURCE` 开启时 `end_function` 打印读它。
pub fn compile_dumped_bcb<'a>(
  source: &str,
  options: &CompileOptions,
  dump_flags: u32,
  dump_source: Option<&str>,
) -> BytecodeBuilder<'a> {
  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(dump_flags);
  if let Some(text) = dump_source {
    bcb.set_dump_source(text);
  }
  let owned = String::from(source);
  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bcb,
    &owned,
    options,
    &ParseOptions::default(),
  );
  bcb
}
