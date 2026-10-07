use alloc::string::String;

use ulua_analysis::{
  functions::get_fragment_region::get_fragment_region,
  records::{arena_handle::Handle, fragment_region::FragmentRegion},
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
    // parse_result.root 是 parse_helper 在 fixture arena 刚分配的存活 AstStatBlock
    // （alloc 恒非空、arena 块不动、比 fixture 长寿）；入口已句柄化，
    // `Handle::from_ptr` 收口非空契约（空 root 不可能出现）。
    get_fragment_region(Handle::from_ptr(parse_result.root), cursor_pos)
  }
}
