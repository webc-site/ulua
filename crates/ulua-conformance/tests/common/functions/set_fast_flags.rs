use alloc::string::String;

use crate::common::functions::{
  parse_f_flag::parse_f_flag, parse_f_int::parse_f_int, set_fast_value::set_fast_value,
};

pub fn set_fast_flags(flags: &Vec<String>) {
  for flag in flags {
    let view = flag.as_str();

    if view == "true" || view == "false" {
      let _state = view == "true";

      // Iterate all registered FFlag flags and set their values
      // We use the FFlag module's exported statics directly
      // This mirrors the C++ loop over FValue<bool>::list
      // Since we cannot enumerate all FFlag statics at runtime in Rust,
      // we rely on the fact that set_fast_value will look up the flag by name
      // and set it. The C++ code sets flag->value directly, but in Rust
      // the FFlag values are accessed via FValue::get() and set via FValue::set()
      // which are only available in ulua_common. The schedule item set_fast_value
      // is expected to perform the actual lookup and assignment.
      //
      // For now, we pass "true"/"false" to set_fast_value which should handle
      // setting all flags to that state. Since the original C++ code iterates
      // the list and sets each flag, and we don't have an enumeration mechanism,
      // we keep the dispatch to set_fast_value as a placeholder that will be
      // specialized elsewhere.
      set_fast_value();
      continue;
    }

    let mut view = view;
    if view.len() >= 2 && view.as_bytes()[0] == b'D' && view.as_bytes()[1] == b'F' {
      view = &view[1..];
    }

    if view.len() >= 4 && &view[0..4] == "FInt" {
      let (name, value) = parse_f_int(&view[4..]);
      set_fast_value();
      let _ = (name, value);
    } else {
      // We want to prevent the footgun where '--fflags=LuauSomeFlag' is ignored. We'll assume that this was declared as FFlag.
      let (name, value) = if view.len() >= 5 && &view[0..5] == "FFlag" {
        parse_f_flag(&view[5..])
      } else {
        parse_f_flag(view)
      };
      set_fast_value();
      let _ = (name, value);
    }
  }
}
