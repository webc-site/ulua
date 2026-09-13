use alloc::string::String;

use ulua_common::records::f_value::FValue;

use crate::functions::{
  parse_f_flag::parse_f_flag, parse_f_int::parse_f_int, set_fast_value::set_fast_value,
  skip_fast_flag::skip_fast_flag,
};

/// C++ `static void setFastFlags(const std::vector<doctest::String>& flags)`
/// (tests/main.cpp:324): parse each `--fflags=` token and apply it. A bare
/// `true`/`false` sets every non-skipped bool flag; a leading `DF` prefix is
/// dropped; `FInt...` tokens set an integer flag, all others are treated as
/// `FFlag` bool flags (with the documented footgun guard).
pub fn set_fast_flags(flags: &[String]) {
  for flag in flags {
    let mut view: &str = flag.as_str();

    if view == "true" || view == "false" {
      let value = view == "true";
      FValue::<bool>::set_all_unless(value, skip_fast_flag);
      continue;
    }

    // C++ `view.remove_prefix(1)` when it begins with "DF".
    if view.len() >= 2 && view.as_bytes()[0] == b'D' && view.as_bytes()[1] == b'F' {
      view = &view[1..];
    }

    if view.len() >= 4 && &view[0..4] == "FInt" {
      let (name, value) = parse_f_int(&view[4..]);
      set_fast_value(&name, value);
    } else {
      // Prevent the footgun where '--fflags=LuauSomeFlag' is silently
      // ignored: assume a missing "FFlag" prefix was intended as one.
      let rest = if view.len() >= 5 && &view[0..5] == "FFlag" {
        &view[5..]
      } else {
        view
      };
      let (name, value) = parse_f_flag(rest);
      set_fast_value(&name, value);
    }
  }
}
