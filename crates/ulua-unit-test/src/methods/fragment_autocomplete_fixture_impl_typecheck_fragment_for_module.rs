//! C++ `std::pair<FragmentTypeCheckStatus, FragmentTypeCheckResult>
//! FragmentAutocompleteFixtureImpl::typecheckFragmentForModule(...)`
//! (tests/FragmentAutocomplete.test.cpp:260-268).

use ulua_analysis::{
  enums::fragment_type_check_status::FragmentTypeCheckStatus,
  functions::typecheck_fragment_fragment_autocomplete_alt_b::typecheck_fragment,
  records::{
    fragment_type_check_result::FragmentTypeCheckResult,
    i_fragment_autocomplete_reporter::null_reporter,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::position::Position;

use crate::{
  functions::get_options::get_options,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn typecheck_fragment_for_module(
    &mut self,
    module: &ModuleName,
    document: &str,
    cursor_pos: Position,
    fragment_end_position: Option<Position>,
  ) -> (FragmentTypeCheckStatus, FragmentTypeCheckResult) {
    let pr = self.parse_helper(document.to_owned());
    let options = get_options();
    let frontend = self.base.get_frontend();
    typecheck_fragment(
      frontend,
      module,
      &cursor_pos,
      Some(options),
      document,
      fragment_end_position,
      pr.root,
      null_reporter(),
    )
  }
}
