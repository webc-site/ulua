use ulua_analysis::{
  functions::find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse,
  records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult,
};
use ulua_ast::records::{parse_options::ParseOptions, position::Position};

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn run_autocomplete_visitor(
    &mut self,
    source: &str,
    cursor_pos: &Position,
  ) -> FragmentAutocompleteAncestryResult {
    let parse_result = self.base.base.try_parse(source, &ParseOptions::default());
    assert!(!parse_result.root.is_null());
    unsafe { find_ancestry_for_fragment_parse(parse_result.root, *cursor_pos, parse_result.root) }
  }
}
