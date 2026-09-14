//! C++ `FragmentAutocompleteStatusResult
//! FragmentAutocompleteFixtureImpl::autocompleteFragmentForModule(...)`
//! (tests/FragmentAutocomplete.test.cpp:270-283).
use alloc::boxed::Box;

use ulua_analysis::{
  functions::try_fragment_autocomplete::try_fragment_autocomplete,
  records::{
    fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    fragment_context::FragmentContext, frontend_options::FrontendOptions,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::position::Position;

use crate::{
  functions::null_callback_autocomplete_test::null_callback,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn autocomplete_fragment_for_module(
    &mut self,
    module: &ModuleName,
    document: &str,
    cursor_pos: Position,
    fragment_end_position: Option<Position>,
  ) -> FragmentAutocompleteStatusResult {
    let parse_result = self.parse_helper(document.to_owned());
    // C++ uses a default-constructed FrontendOptions here (not getOptions()).
    let options = FrontendOptions::default();
    let context = FragmentContext::new_with_options(
      document,
      &parse_result,
      Some(options),
      fragment_end_position,
    );
    let frontend = self.base.get_frontend();
    try_fragment_autocomplete(
      frontend,
      module,
      cursor_pos,
      context,
      Box::new(null_callback),
    )
  }
}
