use alloc::vec::Vec;

use ulua_config::records::lint_warning::LintWarning;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LintResult {
  pub errors: Vec<LintWarning>,
  pub warnings: Vec<LintWarning>,
}
