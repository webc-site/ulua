use crate::records::lint_format_string::LintFormatString;

impl LintFormatString {
  pub fn check_date_format(&self, data: &[u8]) -> Option<&'static str> {
    let options = b"aAbBcdHIjmMpSUwWxXyYzZ";
    let size = data.len();

    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;

        if i == size {
          return Some("unfinished replacement");
        }

        let next_ch = data[i];
        if next_ch != b'%' && !options.contains(&next_ch) {
          return Some("unexpected replacement character; must be a date format specifier or %");
        }
      }

      if data[i] == 0 {
        return Some("date format can not contain null characters");
      }
      i += 1;
    }

    None
  }
}
