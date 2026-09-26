use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;

impl<'a> FeedbackVectorFixture<'a> {
  /// cpp `FeedbackVectorFixture::compile`：`Dump_Code` + `optimizationLevel = 0`。
  ///
  /// 源码只被读一次（下游 `compileOrThrow` 收 `const char*`），故这里用 `&str` 而非
  /// cpp 的按值 `std::string`，避免每个用例多一次堆分配。
  pub fn compile(&mut self, source: &str) {
    self.bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let opts = CompileOptions {
      optimization_level: 0,
      ..CompileOptions::default()
    };

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut self.bcb,
      source,
      &opts,
      &ParseOptions::default(),
    );
  }
}
