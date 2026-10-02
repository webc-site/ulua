use core::ptr::from_ref;

use ulua_analysis::{
  functions::find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse,
  records::{
    arena_handle::Handle,
    fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult,
  },
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
    // 引用 → 裸地址视图用 `from_ref + cast_mut`（cpp 把同一 `AstStatBlock*`
    // 作两参数传入的忠实镜像），免 `as *const _ as *mut _` 双重 `as` 反模式。
    let root = from_ref(parse_result.root_block().expect("hard parse error")).cast_mut();
    // 入口已句柄化：root 指向 fixture.allocator 中存活的根块（测试期内存活），
    // cpp 把同一 `AstStatBlock*` 作 stale/lastGoodParse 两参数传入的忠实镜像。
    let root = Handle::from_ptr(root);
    find_ancestry_for_fragment_parse(Some(root), *cursor_pos, Some(root))
  }
}
