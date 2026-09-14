use alloc::format;

use ulua_ast::enums::mode::Mode;

use crate::type_aliases::error::Error;

pub fn parse_mode_string(mode: &mut Mode, mode_string: &str, compat: bool) -> Error {
  match mode_string {
    "nocheck" => *mode = Mode::NoCheck,
    "strict" => *mode = Mode::Strict,
    "nonstrict" => *mode = Mode::Nonstrict,
    // compat 模式兼容旧名 "noinfer"
    "noinfer" if compat => *mode = Mode::NoCheck,
    _ => {
      return Some(format!(
        "Bad mode \"{mode_string}\".  Valid options are nocheck, nonstrict, and strict"
      ));
    }
  }

  None
}
