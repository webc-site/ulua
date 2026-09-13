use ulua_analysis::records::check_result::CheckResult;
use ulua_ast::enums::mode::Mode;
use ulua_common::FFlag;

use crate::{
  functions::get_options::get_options,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn check_old_solver(&mut self, source: &str) -> CheckResult {
    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    self.base.base.check_mode_string_optional_frontend_options(
      Mode::Strict,
      source,
      Some(get_options()),
    )
  }
}
