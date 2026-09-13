use crate::records::lint_format_string::LintFormatString;

impl LintFormatString {
  #[inline]
  pub fn check_string_match_set(
    &self,
    data: &[u8],
    magic: &[u8],
    classes: &[u8],
  ) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;

        if i == size {
          return Some("unfinished character class");
        }

        let next_ch = data[i];
        if self.is_digit(next_ch) {
          return Some("sets can not contain capture references");
        } else if self.is_alpha(next_ch) {
          // lower case lookup - upper case for every character class is defined as its inverse
          if !classes.contains(&(next_ch | b' ')) {
            return Some("invalid character class, must refer to a defined class or its inverse");
          }
        } else {
          // technically % can escape any non-alphanumeric character but this is error-prone
          if !magic.contains(&next_ch) {
            return Some("expected a magic character after %");
          }
        }

        if i + 1 < size && data[i + 1] == b'-' {
          return Some("character range can't include character sets");
        }
      } else if ch == b'-' && i + 1 < size && data[i + 1] == b'%' {
        return Some("character range can't include character sets");
      }

      i += 1;
    }

    None
  }
}
