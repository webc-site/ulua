use ulua_analysis::{
  functions::parse_fragment::parse_fragment,
  records::{arena_handle::Handle, fragment_parse_result::FragmentParseResult},
};
use ulua_ast::records::position::Position;

use crate::{
  functions::{get_options::get_options, raw_handle::raw_handle},
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};
impl FragmentAutocompleteFixtureImpl {
  pub fn parse_fragment(
    &mut self,
    document: &str,
    cursor_pos: &Position,
    fragment_end_position: Option<Position>,
  ) -> Option<FragmentParseResult> {
    let parse_result = self.parse_helper(document.to_owned());
    let module = self
      .base
      .base
      .get_main_module(get_options().for_autocomplete);

    if module.is_null() {
      return None;
    }

    // cpp `parseFragment(module->root, .., module->names.get(), ..)`：`getMainModule()`
    // 给的是 `shared_ptr`，这里只按 cpp 那样**读**它的字段，不伪造独占借用（`&mut Module`
    // 会与 resolver 里那份 `ModulePtr` 冲突）；`names` 会被下游按 `AstNameTable&` 写入，
    // 故仍取 `*mut` 句柄，与 cpp `unique_ptr::get()` 同形。
    // Safety: `module` 由 `get_main_module` 取自 resolver 保有的 `ModulePtr`，
    // 在 `Fixture` 存活期内有效。
    let module_ref = unsafe { &*module };
    let names = raw_handle(module_ref.names.as_ref()?);

    // Safety: 行 33 判空后把模块物化为只读共享借用（resolver ModulePtr 存活，见上方 31-32 行注记），不伪造 &mut；两树根经 `from_opt_ptr` 折叠可空性（root 为 arena 存活 AST 起点、parse_result.root 为本次 parse 的存活产物），names 取 resolver 内 AstNameTable 非空块地址（cpp names.get() 同形，下游按 & 只读使用）；单线程帧内使用。
    unsafe {
      parse_fragment(
        Handle::from_opt_ptr(module_ref.root),
        Handle::from_opt_ptr(parse_result.root),
        names,
        document,
        cursor_pos,
        fragment_end_position,
      )
    }
  }
}
