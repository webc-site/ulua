//! Port of `Fixture::matchParseError` (tests/Fixture.cpp:389). Parses (with
//! declaration syntax enabled), asserts at least one parse error occurred, and
//! that the first error's message — and, when given, its location — match the
//! expected values. Returns the full `ParseResult` like the C++.

use ulua_ast::records::{
  location::Location, parse_options::ParseOptions, parse_result::ParseResult, parser::Parser,
};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn match_parse_error(
    &mut self,
    source: &str,
    message: &str,
    location: Option<Location>,
  ) -> ParseResult {
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
      // C++: CHECK_EQ(result.errors.front().getMessage(), message);
      assert_eq!(first.get_message(), message);

      // C++: if (location) CHECK_EQ(result.errors.front().getLocation(), *location);
      if let Some(loc) = location {
        assert_eq!(*first.get_location(), loc);
      }
    }

    result
  }
}
