use alloc::string::String;

use ulua_analysis::records::source_module::SourceModule;
use ulua_ast::records::parse_result::ParseResult;

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn parse_helper(&mut self, document: String) -> ParseResult {
    let source: *mut SourceModule = self.get_source();
    unsafe { self.parse_helper_(&mut *source, document) }
  }
}
