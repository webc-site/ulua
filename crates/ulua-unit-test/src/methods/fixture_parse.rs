//! Test fixture: port of `Fixture::parse` (tests/Fixture.cpp). Minimal/faithful
//! for the PARSER test suite: it parses into the fixture's own allocator+name
//! table (which outlive the returned AST for the test's duration) and, on parse
//! errors, mirrors the C++ `throw ParseErrors(result.errors)` with a panic that
//! carries the first message. The C++ also runs check()+lint() on error to
//! exercise error nodes through the Analysis frontend; that path is intentionally
//! omitted here (it pulls the whole typechecker and is irrelevant to parser-shape
//! assertions). Error-expectation tests go through `match_parse_error` instead.
use alloc::boxed::Box;
use std::panic::panic_any;

use ulua_analysis::records::source_module::SourceModule;
use ulua_ast::records::{
  ast_stat_block::AstStatBlock, parse_errors::ParseErrors, parse_options::ParseOptions,
  parser::Parser,
};

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn parse(&mut self, source: &str, parse_options: &ParseOptions) -> *mut AstStatBlock {
    // Re-point the name table at the allocator's *current* address: the
    // fixture (and its allocator) may have been moved since construction,
    // which would leave the table's stored allocator pointer dangling.
    self
      .name_table
      .rebind_allocator(&mut self.allocator as *mut _);

    let result = Parser::parse(
      source,
      source.len(),
      &mut self.name_table,
      &mut self.allocator,
      parse_options.clone(),
    );

    let root = result.root;

    // C++ Fixture::parse populates sourceModule->root (and hotcomments) BEFORE
    // it throws, so error-recovery tests can still visit the partial AST after
    // catching the throw. We skip the check()/lint() Analysis pass (irrelevant
    // to parser-shape assertions). The AST lives in self.allocator, which
    // outlives the test, so storing the raw root pointer here is sound.
    let mut sm = SourceModule::new();
    sm.root = root;
    sm.hotcomments = result.hotcomments.clone();
    self.source_module = Some(Box::new(sm));

    if !result.errors.is_empty() {
      // Faithful to C++ `throw ParseErrors(result.errors)`: panic with the
      // ParseErrors payload so callers can downcast it (e.g. checkRecovery).
      panic_any(ParseErrors::new(result.errors));
    }

    root
  }
}
