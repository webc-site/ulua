use crate::records::lint_options::LintOptions;

impl LintOptions {
  pub fn set_defaults(&mut self) {
    // 默认启用全部警告
    self.warning_mask = !0u64;
  }
}
