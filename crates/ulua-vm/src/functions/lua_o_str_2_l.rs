use core::{
  ffi::c_char,
  ptr::{eq, null_mut},
};

use crate::macros::luai_str_2_long::strtoll;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_o_str_2_l(s: *const c_char, result: *mut i64, base: i32) -> i32 {
  unsafe {
    let mut endptr: *mut c_char = null_mut();

    if base == 10 {
      *result = strtoll(s, &mut endptr, base) as i64;
      if eq(endptr, s) {
        return 0; // conversion failed
      }
      if *endptr == b'x' as c_char || *endptr == b'X' as c_char {
        // maybe an hexadecimal constant?
        *result = strtoull(s, &mut endptr, 16) as i64;
      }
    } else {
      // unsigned parse in other bases
      *result = strtoull(s, &mut endptr, base as u32) as i64;
      if eq(endptr, s) {
        return 0;
      }
    }

    if *endptr == b'\0' as c_char {
      return 1; // most common case
    }

    while isspace(*endptr as u8) {
      endptr = endptr.add(1);
    }

    if *endptr != b'\0' as c_char {
      return 0; // invalid trailing characters?
    }

    1
  }
}

// Helper for isspace: check if a u32 value corresponds to an ASCII whitespace character
#[inline]
fn isspace(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// Helper function for strtoull-like behavior via libc-compatible symbol
unsafe fn strtoull(s: *const c_char, endptr: &mut *mut c_char, base: u32) -> u64 {
  unsafe {
    unsafe extern "C" {
      fn strtoull(s: *const c_char, endptr: *mut *mut c_char, base: u32) -> u64;
    }
    strtoull(s, endptr as *mut *mut c_char, base)
  }
}
