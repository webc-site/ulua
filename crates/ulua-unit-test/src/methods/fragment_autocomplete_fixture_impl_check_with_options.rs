use ulua_analysis::records::check_result::CheckResult;

use crate::{
  functions::get_options::get_options,
  records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl,
};

impl FragmentAutocompleteFixtureImpl {
  pub fn check_with_options(&mut self, source: &str) -> CheckResult {
    self.base.get_frontend();
    self
      .base
      .base
      .check_string_optional_frontend_options(source, Some(get_options()))
  }
}
