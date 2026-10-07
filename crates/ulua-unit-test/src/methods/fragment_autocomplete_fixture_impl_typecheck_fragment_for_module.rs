//! C++ `std::pair<FragmentTypeCheckStatus, FragmentTypeCheckResult>
//! FragmentAutocompleteFixtureImpl::typecheckFragmentForModule(...)`
//! (tests/FragmentAutocomplete.test.cpp:260-268).

use ulua_analysis::{
  enums::fragment_type_check_status::FragmentTypeCheckStatus,
  functions::typecheck_fragment_fragment_autocomplete::typecheck_fragment,
  records::{
    arena_handle::Handle, fragment_type_check_result::FragmentTypeCheckResult,
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
      // cpp 直传解析根（可 null）：`from_opt_ptr` 折叠可空性。
      Handle::from_opt_ptr(pr.root),
      ReporterRef::NULL,
    )
  }
}
