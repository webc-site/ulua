use alloc::sync::Arc;

use ulua_analysis::{
  functions::parse_fragment::parse_fragment, records::fragment_parse_result::FragmentParseResult,
};
use ulua_ast::records::position::Position;

use crate::{
  functions::get_options::get_options,
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

    let module_ref = unsafe { &mut *module };
    let names = Arc::as_ptr(module_ref.names.as_ref()?) as *mut _;

    unsafe {
      parse_fragment(
        module_ref.root,
        parse_result.root,
        names,
        document,
        cursor_pos,
        fragment_end_position,
      )
    }
  }
}
