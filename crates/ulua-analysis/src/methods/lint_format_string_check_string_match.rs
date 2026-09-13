use alloc::vec::Vec;

use crate::records::lint_format_string::LintFormatString;
impl LintFormatString {
  #[inline]
  pub fn check_string_match(&self, data: &[u8]) -> Result<i32, &'static str> {
    let magic = b"^$()%.[]*+-?)";
    let classes = b"acdglpsuwxz";
    let size = data.len();

    let mut open_captures: Vec<i32> = Vec::new();
    let mut total_captures: i32 = 0;

    let mut i: usize = 0;
    while i < size {
      if data[i] == b'%' {
        i += 1;

        if i == size {
          return Err("unfinished character class");
        }

        let ch = data[i];
        if self.is_digit(ch) {
          if ch == b'0' {
            return Err("invalid capture reference, must be 1-9");
          }

          let capture_index = (ch - b'0') as i32;

          if capture_index > total_captures {
            return Err("invalid capture reference, must refer to a valid capture");
          }

          for &open in &open_captures {
            if open == capture_index {
              return Err("invalid capture reference, must refer to a closed capture");
            }
          }
        } else if self.is_alpha(ch) {
          if ch == b'b' {
            if i + 2 >= size {
              return Err("missing brace characters for balanced match");
            }
            i += 2;
          } else if ch == b'f' {
            if i + 1 >= size || data[i + 1] != b'[' {
              return Err("missing set after a frontier pattern");
            }
            // we can parse the set with the regular logic
          } else {
            // lower case lookup - upper case for every character class is defined as its inverse
            if !classes.contains(&(ch | b' ')) {
              return Err("invalid character class, must refer to a defined class or its inverse");
            }
          }
        } else {
          // technically % can escape any non-alphanumeric character but this is error-prone
          if !magic.contains(&ch) {
            return Err("expected a magic character after %");
          }
        }
      } else if data[i] == b'[' {
        let mut j = i + 1;

        // empty patterns don't exist as per grammar rules, so we skip leading ^ and ]
        if j < size && data[j] == b'^' {
          j += 1;
        }
        if j < size && data[j] == b']' {
          j += 1;
        }

        // scan for the end of the pattern
        while j < size && data[j] != b']' {
          // % escapes the next character
          if j + 1 < size && data[j] == b'%' {
            j += 1;
          }
          j += 1;
        }

        if j == size {
          return Err("expected ] at the end of the string to close a set");
        }

        if let Some(error) = self.check_string_match_set(&data[i + 1..j], magic, classes) {
          return Err(error);
        }

        debug_assert!(data[j] == b']');
        i = j;
      } else if data[i] == b'(' {
        total_captures += 1;
        open_captures.push(total_captures);
      } else if data[i] == b')' {
        if open_captures.is_empty() {
          return Err("unexpected ) without a matching (");
        }
        open_captures.pop();
      }

      i += 1;
    }

    if !open_captures.is_empty() {
      return Err("expected ) at the end of the string to close a capture");
    }

    Ok(total_captures)
  }
}
