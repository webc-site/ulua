//! Port of `Fixture::matchParseError` (tests/Fixture.cpp:389). Parses (with
//! declaration syntax enabled), asserts at least one parse error occurred, and
//! that the first error's message — and, when given, its location — match the
//! expected values. Returns the full `ParseResult` like the C++.

use ulua_ast::records::{
  location::Location, parse_options::ParseOptions, parse_result::ParseResult, parser::Parser,
};

use crate::records::fixture::Fixture;

impl Fixture {
  /// cpp `matchParseError(source: std::string_view, …)` 的字节入口：源文本允许
  /// 是任意字节序列（`\xFF`、截断的多字节序列等），用于驱动词法层的
  /// "invalid UTF-8 sequence" 分支；内部全程按 &[u8] 处理，不伪造 &str。
  pub fn match_parse_error_bytes(
    &mut self,
    source: &[u8],
    message: &str,
    location: Option<Location>,
  ) -> ParseResult {
    self.match_parse_error_impl(source, message, location)
  }

  pub fn match_parse_error(
    &mut self,
    source: &str,
    message: &str,
    location: Option<Location>,
  ) -> ParseResult {
    self.match_parse_error_impl(source.as_bytes(), message, location)
  }

  fn match_parse_error_impl(
    &mut self,
    source: &[u8],
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
      &mut self.name_table,
      &mut self.allocator,
      options,
    );

    // C++: CHECK_MESSAGE(!result.errors.empty(), "Expected a parse error in '" << source << "'");
    assert!(
      !result.errors.is_empty(),
      "Expected a parse error in '{}'",
      String::from_utf8_lossy(source)
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
