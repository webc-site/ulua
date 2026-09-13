use crate::records::lint_options::LintOptions;

impl LintOptions {
  pub fn set_defaults(&mut self) {
    // By default, we enable all warnings
    self.warning_mask = !0u64;
  }
}
