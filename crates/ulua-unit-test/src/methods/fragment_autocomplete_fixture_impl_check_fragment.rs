//! C++ `FragmentTypeCheckResult FragmentAutocompleteFixtureImpl::checkFragment(...)`
//! (tests/FragmentAutocomplete.test.cpp:161-170).

use ulua_analysis::{
  functions::typecheck_fragment_fragment_autocomplete::typecheck_fragment,
  records::{
    fragment_type_check_result::FragmentTypeCheckResult,
    i_fragment_autocomplete_reporter::ReporterRef,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::position::Position;

use crate::{
  functions::get_options::get_options,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn check_fragment(
    &mut self,
    document: &str,
    cursor_pos: Position,
    fragment_end_position: Option<Position>,
  ) -> FragmentTypeCheckResult {
    let p = self.parse_helper(document.to_owned());
    let options = get_options();
    let frontend = self.base.get_frontend();
    let (_, result) = typecheck_fragment(
      frontend,
      &ModuleName::from("MainModule"),
      &cursor_pos,
      Some(options),
      document,
      fragment_end_position,
      p.root,
      // C++ passes `nullptr` for the reporter.
      ReporterRef::NULL,
    );
    result
  }
}
