use alloc::format;

use ulua_ast::enums::mode::Mode;

use crate::type_aliases::error::Error;

pub fn parse_mode_string(mode: &mut Mode, mode_string: &str, compat: bool) -> Error {
  if mode_string == "nocheck" {
    *mode = Mode::NoCheck;
  } else if mode_string == "strict" {
    *mode = Mode::Strict;
  } else if mode_string == "nonstrict" {
    *mode = Mode::Nonstrict;
  } else if mode_string == "noinfer" && compat {
    *mode = Mode::NoCheck;
  } else {
    return Some(format!(
      "Bad mode \"{}\".  Valid options are nocheck, nonstrict, and strict",
      mode_string
    ));
  }

  None
}
