//! Source: `tests/Fixture.cpp`

use ulua_analysis::{records::lint_result::LintResult, type_aliases::module_name_type::ModuleName};
use ulua_ast::enums::mode::Mode;
use ulua_config::records::lint_options::LintOptions;

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn lint(&mut self, source: &str, lint_options: Option<LintOptions>) -> LintResult {
    let module_name = ModuleName::from(MAIN_MODULE_NAME);
    self.config_resolver.default_config.mode = Mode::Strict;
    self
      .file_resolver
      .source
      .insert(module_name.clone(), source.to_owned());

    let frontend = self.get_frontend();
    frontend.mark_dirty(&module_name, None);

    self.lint_module(&module_name, lint_options)
  }
}
