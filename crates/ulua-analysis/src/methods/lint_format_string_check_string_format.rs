use crate::records::lint_format_string::LintFormatString;

impl LintFormatString {
  pub fn check_string_format(&self, data: &[u8]) -> Option<&'static str> {
    let flags = b"-+ #0";
    let options = b"cdiouxXeEfgGqs*";
    let size = data.len();

    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;

        if i < size && data[i] == b'%' {
          i += 1;
          continue;
        }

        while i < size && flags.contains(&data[i]) {
          i += 1;
        }

        if i < size && self.is_digit(data[i]) {
          i += 1;
        }
        if i < size && self.is_digit(data[i]) {
          i += 1;
        }

        if i < size && data[i] == b'.' {
          i += 1;

          if i < size && self.is_digit(data[i]) {
            i += 1;
          }
          if i < size && self.is_digit(data[i]) {
            i += 1;
          }
        }

        if i == size {
          return Some("unfinished format specifier");
        }

        if !options.contains(&data[i]) {
          return Some("invalid format specifier: must be a string format specifier or %");
        }
      }
      i += 1;
    }

    None
  }
}
