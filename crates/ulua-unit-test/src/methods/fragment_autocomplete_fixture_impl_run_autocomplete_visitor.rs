use ulua_analysis::{
  functions::find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse,
  records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult,
};
use ulua_ast::records::{
  ast_stat_block::AstStatBlock, parse_options::ParseOptions, position::Position,
};

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn run_autocomplete_visitor(
    &mut self,
    source: &str,
    cursor_pos: &Position,
  ) -> FragmentAutocompleteAncestryResult {
    let parse_result = self.base.base.try_parse(source, &ParseOptions::default());
    let root = parse_result.root_block().expect("hard parse error") as *const AstStatBlock
      as *mut AstStatBlock;
    // SAFETY: root 指向 fixture.allocator 中存活的根块，测试期内存活。
    unsafe { find_ancestry_for_fragment_parse(root, *cursor_pos, root) }
  }
}
