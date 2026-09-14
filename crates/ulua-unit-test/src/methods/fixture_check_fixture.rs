use alloc::string::String;

use ulua_analysis::records::{check_result::CheckResult, frontend_options::FrontendOptions};
use ulua_ast::enums::mode::Mode;

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn check_mode_string_optional_frontend_options(
    &mut self,
    mode: Mode,
    source: &str,
    options: Option<FrontendOptions>,
  ) -> CheckResult {
    self.get_frontend();

    let module_name = String::from(MAIN_MODULE_NAME);
    self.config_resolver.default_config.mode = mode;
    self
      .file_resolver
      .source
      .insert(module_name.clone(), source.to_owned());

    let frontend = self.get_frontend();
    frontend.mark_dirty(&module_name, None);
    frontend.clear_stats();
    frontend.check_module_name_optional_frontend_options(&module_name, options)
  }
}
