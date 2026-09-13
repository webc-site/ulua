use alloc::string::String;

use ulua_analysis::{
  functions::get_fragment_region::get_fragment_region, records::fragment_region::FragmentRegion,
};
use ulua_ast::records::{parse_result::ParseResult, position::Position};

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn get_autocomplete_region(
    &mut self,
    source: String,
    cursor_pos: &Position,
  ) -> FragmentRegion {
    let parse_result: ParseResult = self.parse_helper(source);
    unsafe { get_fragment_region(parse_result.root, cursor_pos) }
  }
}
