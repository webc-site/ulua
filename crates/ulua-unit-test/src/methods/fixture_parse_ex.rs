use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn parse_ex(&mut self, source: &str, options: &ParseOptions) -> ParseResult {
    let result = self.try_parse(source, options);
    if !result.errors.is_empty() {
      panic!("ParseErrors: {:?}", result.errors);
    }
    result
  }
}
