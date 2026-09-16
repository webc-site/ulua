use ulua_ast::records::{
  allocator::Allocator, ast_stat_block::AstStatBlock, parse_options::ParseOptions, parser::Parser,
};

use crate::records::require_tracer_fixture::RequireTracerFixture;

impl RequireTracerFixture {
  pub fn parse(&mut self, src: &str) -> *mut AstStatBlock {
    self
      .names
      .rebind_allocator(&mut *self.allocator as *mut Allocator);

    let result = Parser::parse(
      src,
      src.len(),
      &mut self.names,
      &mut self.allocator,
      ParseOptions::default(),
    );

    assert!(
      result.errors.is_empty(),
      "Parse error: {}",
      result
        .errors
        .iter()
        .map(|error| error.what())
        .collect::<Vec<_>>()
        .join("\n")
    );

    result.root
  }
}
