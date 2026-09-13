use alloc::{string::String, sync::Arc};

use ulua_analysis::records::source_module::SourceModule;
use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult, parser::Parser};

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
impl FragmentAutocompleteFixtureImpl {
  pub fn parse_helper_(&mut self, source: &mut SourceModule, document: String) -> ParseResult {
    let parse_options = ParseOptions {
      capture_comments: true,
      ..Default::default()
    };
    let allocator = Arc::get_mut(&mut source.allocator)
      .expect("fresh fragment source allocator must be unique") as *mut _;
    let names =
      Arc::get_mut(&mut source.names).expect("fresh fragment source name table must be unique");
    names.rebind_allocator(allocator);

    let parse_result = unsafe {
      Parser::parse(
        document.as_str(),
        document.len(),
        names,
        &mut *allocator,
        parse_options,
      )
    };

    source.parse_errors = parse_result.errors.clone();
    source.root = parse_result.root;
    source.hotcomments = parse_result.hotcomments.clone();
    source.comment_locations = parse_result.comment_locations.clone();

    parse_result
  }
}
