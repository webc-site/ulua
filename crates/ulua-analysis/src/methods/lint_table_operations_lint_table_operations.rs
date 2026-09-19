use crate::records::{lint_context::LintContext, lint_table_operations::LintTableOperations};

impl LintTableOperations {
  pub fn new(context: *mut LintContext) -> Self {
    LintTableOperations { context }
  }
}
