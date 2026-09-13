use crate::{enums::code::Code, records::lint_options::LintOptions};

impl LintOptions {
  pub fn enable_warning(&mut self, code: Code) {
    self.warning_mask |= 1u64 << (code as i32);
  }
}
