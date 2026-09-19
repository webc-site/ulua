//! Port of `Fixture::matchParseErrorPrefix` (tests/Fixture.cpp:410). Parses (with
//! declaration syntax enabled), asserts at least one parse error occurred, and
//! that the first error's message STARTS WITH the expected prefix. Returns the
//! full `ParseResult` like the C++.

use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn match_parse_error_prefix(&mut self, source: &str, prefix: &str) -> ParseResult {
    let options = ParseOptions {
      allow_declaration_syntax: true,
      ..Default::default()
    };
    self
      .name_table
      .rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      source,
      source.len(),
      &mut self.name_table,
      &mut self.allocator,
      options,
    );

    // C++: CHECK_MESSAGE(!result.errors.empty(), "Expected a parse error in '" << source << "'");
    assert!(
      !result.errors.is_empty(),
      "Expected a parse error in '{source}'"
    );

    if let Some(first) = result.errors.first() {
      let message = first.get_message();
      // C++: CHECK_GE(message.length(), prefix.length()); CHECK_EQ(prefix, message.substr(0, prefix.size()));
      assert!(message.len() >= prefix.len());
      assert_eq!(prefix, &message[..prefix.len()]);
    }

    result
  }
}
