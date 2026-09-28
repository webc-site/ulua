//! C++ `std::string FragmentAutocompleteFixtureImpl::cleanMarkers(const std::string& source)`
//! (tests/FragmentAutocomplete.test.cpp:69-107).
use alloc::string::String;

use ulua_ast::records::position::Position;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;

impl FragmentAutocompleteFixtureImpl {
  pub fn clean_markers(&mut self, source: &str) -> String {
    self.marker_position.clear();
    let mut filtered_source = String::with_capacity(source.len());

    let mut cur_pos = Position { line: 0, column: 0 };
    let mut prev_char: char = '\0';

    for c in source.chars() {
      let mut c = c;
      if prev_char == '@' {
        LUAU_ASSERT!(
          c.is_ascii_digit() || c.is_ascii_uppercase(),
          "Illegal marker character"
        );
        LUAU_ASSERT!(
          !self.marker_position.contains_key(&c),
          "Duplicate marker found"
        );
        self.marker_position.insert(c, cur_pos);
      } else if c == '@' {
        // skip the '@' character
        if prev_char == '\\' {
          // escaped @, prevent prevChar to be equal to '@' on next loop
          c = '\0';
          // replace escaping '\' with '@'
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
    LUAU_ASSERT!(prev_char != '@', "Digit expected after @ symbol");

    filtered_source
  }
}
