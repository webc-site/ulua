use crate::records::lint_format_string::LintFormatString;
impl LintFormatString {
  #[inline]
  pub fn is_alpha(&self, ch: u8) -> bool {
    ((ch | b' ').wrapping_sub(b'a')) < 26
  }
}
