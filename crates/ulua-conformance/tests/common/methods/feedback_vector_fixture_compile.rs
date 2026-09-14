use alloc::string::String;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_compiler::{
  functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;

impl FeedbackVectorFixture {
  pub fn compile(&mut self, source: String) {
    self.bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let opts = CompileOptions {
      optimization_level: 0,
      ..CompileOptions::default()
    };

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut self.bcb,
      &source,
      &opts,
      &ParseOptions::default(),
    );
  }
}
