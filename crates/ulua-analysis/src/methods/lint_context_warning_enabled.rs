use ulua_config::enums::code::Code;

use crate::records::lint_context::LintContext;
impl LintContext {
  pub fn warning_enabled(&mut self, code: Code) -> bool {
    let code_val = code as u64;
    (self.options.warning_mask & (1u64 << code_val)) != 0
  }
}
