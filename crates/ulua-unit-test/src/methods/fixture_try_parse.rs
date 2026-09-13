//! Port of `Fixture::tryParse` (tests/Fixture.cpp:378). Unlike `parse`, this does
//! NOT throw/panic on parse errors — error-recovery tests use it to exercise the
//! parser and then inspect `result.errors` themselves. It enables declaration
//! syntax and parses into the fixture's own allocator + name table (which outlive
//! the returned AST for the test's duration).

use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn try_parse(&mut self, source: &str, parse_options: &ParseOptions) -> ParseResult {
    let mut options: ParseOptions = parse_options.clone();
    options.allow_declaration_syntax = true;

    // See `fixture_parse.rs` — keep the name table pointed at the live allocator.
    self
      .name_table
      .rebind_allocator(&mut self.allocator as *mut _);

    Parser::parse(
      source,
      source.len(),
      &mut self.name_table,
      &mut self.allocator,
      options,
    )
  }
}
