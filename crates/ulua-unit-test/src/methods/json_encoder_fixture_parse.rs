use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser};

use crate::records::json_encoder_fixture::JsonEncoderFixture;

impl JsonEncoderFixture {
  pub fn parse(&mut self, src: &str) -> ParseResult {
    let options = ParseOptions {
      allow_declaration_syntax: true,
      ..Default::default()
    };
    self.names.rebind_allocator(&mut self.allocator as *mut _);

    Parser::parse(
      src,
      src.len(),
      &mut self.names,
      &mut self.allocator,
      options,
    )
  }
}
