//! C++ `void FragmentAutocompleteFixtureImpl::autocompleteFragmentInOldSolver(...)`
//! (tests/FragmentAutocomplete.test.cpp:211-228).
use alloc::boxed::Box;

use ulua_analysis::{
  enums::{fragment_autocomplete_status::FragmentAutocompleteStatus, solver_mode::SolverMode},
  records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
};
use ulua_ast::records::position::Position;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::get_options::get_options,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn autocomplete_fragment_in_old_solver(
    &mut self,
    document: &str,
    updated: &str,
    marker: char,
    assertions: Box<dyn Fn(&mut FragmentAutocompleteStatusResult)>,
    fragment_end_position: Option<Position>,
  ) {
    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let _clean_document = self.clean_markers(document);
    let clean_updated = self.clean_markers(updated);
    let cursor_pos = self.get_position(marker);

    self
      .base
      .get_frontend()
      .set_luau_solver_mode(SolverMode::Old);
    self
      .base
      .base
      .check_string_optional_frontend_options(document, Some(get_options()));

    let mut result = self.autocomplete_fragment(&clean_updated, cursor_pos, fragment_end_position);
    LUAU_ASSERT!(result.status != FragmentAutocompleteStatus::InternalIce);
    assertions(&mut result);
  }
}
