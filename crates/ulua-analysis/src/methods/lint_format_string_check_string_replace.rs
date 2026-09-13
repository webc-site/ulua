use crate::records::lint_format_string::LintFormatString;
impl LintFormatString {
  #[inline]
  pub fn check_string_replace(&self, data: &[u8], captures: i32) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      if data[i] == b'%' {
        i += 1;

        if i == size {
          return Some("unfinished replacement");
        }

        let next_ch = data[i];
        if next_ch != b'%' && !self.is_digit(next_ch) {
          return Some("unexpected replacement character; must be a digit or %");
        }

        if self.is_digit(next_ch) && captures >= 0 && (next_ch - b'0') as i32 > captures {
          return Some("invalid capture index, must refer to pattern capture");
        }
      }

      i += 1;
    }

    None
  }
}
