//! C++ `FragmentAutocompleteStatusResult FragmentAutocompleteFixtureImpl::autocompleteFragment(...)`
//! (tests/FragmentAutocomplete.test.cpp:172-184).
use alloc::{boxed::Box, string::String};

use ulua_analysis::{
  functions::try_fragment_autocomplete::try_fragment_autocomplete,
  records::{
    fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    fragment_context::FragmentContext,
  },
};
use ulua_ast::records::position::Position;

use crate::{
  functions::{get_options::get_options, null_callback_autocomplete_test::null_callback},
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn autocomplete_fragment(
    &mut self,
    document: &str,
    cursor_pos: Position,
    fragment_end_position: Option<Position>,
  ) -> FragmentAutocompleteStatusResult {
    let parse_result = self.parse_helper(document.to_owned());
    let options = get_options();
    let context = FragmentContext::new_with_options(
      document,
      &parse_result,
      Some(options),
      fragment_end_position,
    );
    let frontend = self.base.get_frontend();
    try_fragment_autocomplete(
      frontend,
      &String::from("MainModule"),
      cursor_pos,
      context,
      Box::new(null_callback),
    )
  }
}
