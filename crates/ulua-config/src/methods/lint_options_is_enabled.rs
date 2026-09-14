use crate::{enums::code::Code, records::lint_options::LintOptions};

impl LintOptions {
  pub fn is_enabled(&self, code: Code) -> bool {
    self.warning_mask & code.mask_bit() != 0
  }
}
