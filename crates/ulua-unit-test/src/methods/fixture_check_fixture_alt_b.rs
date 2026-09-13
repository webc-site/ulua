use ulua_analysis::records::{check_result::CheckResult, frontend_options::FrontendOptions};
use ulua_ast::enums::mode::Mode;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn check_string_optional_frontend_options(
    &mut self,
    source: &str,
    options: Option<FrontendOptions>,
  ) -> CheckResult {
    self.check_mode_string_optional_frontend_options(Mode::Strict, source, options)
  }
}
