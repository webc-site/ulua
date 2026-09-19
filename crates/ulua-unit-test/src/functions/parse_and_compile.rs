//! 解析 + 编译的共用前置：返回已 finalize、开了 `Dump_Code` 的 `BytecodeBuilder`。
//! 供 `BytecodeCompilerFixture` / `BytecodeInlinerFixture` 的取字节码方法复用，
//! 避免三处重复同一整段 arena/parse/compile 样板。
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_options::CompileOptions,
};

/// 解析失败或编译抛错时返回 `None`（cpp 里由 `REQUIRE` / `printf` 暴露，测试侧统一为 Option）。
pub fn parse_and_compile(src: &str, optimization_level: i32) -> Option<BytecodeBuilder> {
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(
    src,
    src.len(),
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  );

  if !result.errors.is_empty() {
    return None;
  }

  let mut bcb = BytecodeBuilder::new(None);
  bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

  let opts = CompileOptions {
    optimization_level,
    ..Default::default()
  };
  let compile_result = catch_unwind(AssertUnwindSafe(|| {
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb, &result, &mut names, &opts,
    );
  }));

  if compile_result.is_err() {
    return None;
  }

  Some(bcb)
}
