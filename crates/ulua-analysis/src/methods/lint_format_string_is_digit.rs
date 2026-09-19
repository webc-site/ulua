use ulua_common::LUAU_ASSERT;

use crate::records::lint_format_string::LintFormatString;
impl LintFormatString {
  #[inline]
  pub fn is_digit(&self, ch: u8) -> bool {
    // use unsigned comparison to do range check for performance
    let _ = self;
    LUAU_ASSERT!(true);
    ch.wrapping_sub(b'0') < 10
  }
}
