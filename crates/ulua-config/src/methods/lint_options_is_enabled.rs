use crate::{enums::code::Code, records::lint_options::LintOptions};

impl LintOptions {
  pub fn is_enabled(&self, code: Code) -> bool {
    0 != (self.warning_mask & (1u64 << (code as i32)))
  }
}
