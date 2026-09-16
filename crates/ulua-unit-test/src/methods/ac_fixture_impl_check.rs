use alloc::string::String;
use core::ffi::c_char;

use ulua_analysis::records::check_result::CheckResult;
use ulua_ast::{enums::mode::Mode, records::position::Position};

use crate::records::ac_fixture_impl::AcFixtureImpl;
impl AcFixtureImpl {
  pub fn check(&mut self, source: impl AsRef<str>) -> CheckResult {
    let source = source.as_ref();
    self.get_frontend();
    self.marker_position.clear();

    let mut filtered_source = String::with_capacity(source.len());
    let mut cur_pos = Position { line: 0, column: 0 };
    let mut prev_char = '\0';

    for mut c in source.chars() {
      if prev_char == '@' {
        assert!(
          c.is_ascii_digit() || c.is_ascii_uppercase(),
          "Illegal marker character"
        );

        let marker = c as u8 as c_char;
        assert!(
          self.marker_position.insert(marker, cur_pos).is_none(),
          "Duplicate marker found"
        );
      } else if c == '@' {
        if prev_char == '\\' {
          c = '\0';
          filtered_source.pop();
          filtered_source.push('@');
        }
      } else {
        filtered_source.push(c);
        if c == '\n' {
          cur_pos.line += 1;
          cur_pos.column = 0;
        } else {
          cur_pos.column += 1;
        }
      }

      prev_char = c;
    }

    assert_ne!(prev_char, '@', "Digit expected after @ symbol");

    self
      .base
      .check_mode_string_optional_frontend_options(Mode::NoCheck, &filtered_source, None)
  }
}
