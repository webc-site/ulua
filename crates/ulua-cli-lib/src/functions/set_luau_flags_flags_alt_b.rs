use core::ffi::{CStr, c_char};

use crate::functions::{set_luau_flag::set_luau_flag, set_luau_flags_flags::set_luau_flags_bool};

pub fn set_luau_flags(list: &str) {
  for part in list.split(',') {
    if part.is_empty() {
      continue;
    }
    if let Some((key, value)) = part.split_once('=') {
      if value == "true" || value == "True" {
        set_luau_flag(key, true);
      } else if value == "false" || value == "False" {
        set_luau_flag(key, false);
      } else {
        eprintln!("Warning: unrecognized value '{value}' for flag '{key}'.");
      }
    } else if part == "true" || part == "True" {
      set_luau_flags_bool(true);
    } else if part == "false" || part == "False" {
      set_luau_flags_bool(false);
    } else {
      set_luau_flag(part, true);
    }
  }
}

/// # Safety
///
/// `list` 必须为 null 或有效 C 字符串
pub unsafe fn set_luau_flags_c_char(list: *const c_char) {
  if list.is_null() {
    return;
  }
  let s = unsafe { CStr::from_ptr(list) }.to_str().unwrap_or("");
  set_luau_flags(s);
}
