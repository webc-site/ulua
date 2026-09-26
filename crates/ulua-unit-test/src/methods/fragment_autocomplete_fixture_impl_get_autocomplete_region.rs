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
    // Safety: parse_result.root 是 parse_helper 在 fixture arena 刚分配的存活 AstStatBlock（alloc 恒非空、arena 块不动、比 fixture 长寿），get_fragment_region 按 cpp 契约只读遍历定区，空 root 不可能出现。
    unsafe { get_fragment_region(parse_result.root, cursor_pos) }
  }
}
