use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_options::CompileOptions,
};

use crate::records::{bytecode_inliner_fixture::BytecodeInlinerFixture, bytecode_res::BytecodeRes};
impl BytecodeInlinerFixture {
  pub fn get_function_bytecode(
    &mut self,
    src: &str,
    optimization_level: i32,
  ) -> Option<BytecodeRes> {
    let mut allocator = Allocator::new();
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

    Some(BytecodeRes {
      inlinee_bytecode: bcb.get_function_data(0),
      caller_bytecode: bcb.get_function_data(1),
      string_table: self.extract_string_table(&bcb),
    })
  }
}
