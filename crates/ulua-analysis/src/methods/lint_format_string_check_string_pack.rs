use crate::records::lint_format_string::LintFormatString;

impl LintFormatString {
  #[inline]
  pub fn check_string_pack(&self, data: &[u8], fixed: bool) -> Option<&'static str> {
    let options = b"<>!=bBhHlLjJTiIfdnczsxX ";
    let unsized_opts = b"<>!zX ";
    let size = data.len();

    let mut i = 0;
    while i < size {
      let ch = data[i];

      if !options.contains(&ch) {
        return Some("unexpected character; must be a pack specifier or space");
      }

      if ch == b'c' && (i + 1 == size || !self.is_digit(data[i + 1])) {
        return Some("fixed-sized string format must specify the size");
      }

      if ch == b'X' && (i + 1 == size || unsized_opts.contains(&data[i + 1])) {
        return Some("X must be followed by a size specifier");
      }

      if fixed && (ch == b'z' || ch == b's') {
        return Some("pack specifier must be fixed-size");
      }

      if (ch == b'!' || ch == b'i' || ch == b'I' || ch == b'c' || ch == b's')
        && i + 1 < size
        && self.is_digit(data[i + 1])
      {
        let isc = ch == b'c';

        let mut v: u32 = 0;
        while i + 1 < size && self.is_digit(data[i + 1]) && v <= (i32::MAX as u32 - 9) / 10 {
          let digit_ch = data[i + 1];
          v = v * 10 + (digit_ch - b'0') as u32;
          i += 1;
        }

        if i + 1 < size && self.is_digit(data[i + 1]) {
          return Some("size specifier is too large");
        }

        if !isc && (v == 0 || v > 16) {
          return Some("integer size must be in range [1,16]");
        }
      }

      i += 1;
    }

    None
  }
}
