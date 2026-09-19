//! Source: `tests/Fixture.cpp`

use ulua_analysis::{records::lint_result::LintResult, type_aliases::module_name_type::ModuleName};
use ulua_config::records::lint_options::LintOptions;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn lint_module(
    &mut self,
    module_name: &ModuleName,
    lint_options: Option<LintOptions>,
  ) -> LintResult {
    let mut options = self.get_frontend().options.clone();
    options.run_lint_checks = true;
    options.enabled_lint_warnings = lint_options;

    self
      .get_frontend()
      .check_module_name_optional_frontend_options(module_name, Some(options))
      .lint_result
  }
}
