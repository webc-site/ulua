use ulua_config::records::{config::Config, lint_warning::LintWarning};

use crate::records::{frontend::Frontend, lint_result::LintResult};

impl Frontend {
  pub fn classify_lints(&self, warnings: &[LintWarning], config: &Config) -> LintResult {
    let mut result = LintResult::default();

    for w in warnings.iter() {
      let should_error = config.lint_errors || config.fatal_lint.is_enabled(w.code);
      if should_error {
        result.errors.push(w.clone());
      } else {
        result.warnings.push(w.clone());
      }
    }

    result
  }
}
