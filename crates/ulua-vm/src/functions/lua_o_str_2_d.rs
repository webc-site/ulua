use core::{
  ffi::{c_char, c_int, c_ulong},
  ptr::{eq, null_mut},
};

use crate::macros::{cast_num::cast_num, luai_str_2_num::luai_str2num};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_o_str_2_d(s: *const c_char, result: *mut f64) -> i32 {
  let mut endptr: *mut c_char = null_mut();
  unsafe {
    *result = luai_str2num!(s, &mut endptr);
  }
  if eq(endptr, s) {
    return 0; // conversion failed
  }
  unsafe {
    if *endptr == b'x' as c_char || *endptr == b'X' as c_char {
      // maybe an hexadecimal constant?
      *result = cast_num!(strtoul(s, &mut endptr, 16));
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
  }
  1
}

// Helper functions for C standard library functions not directly available in core
unsafe fn strtoul(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
  unsafe {
    unsafe extern "C" {
      fn strtoul(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    }
    strtoul(s, endptr, base)
  }
}

// Helper for isspace: check if a u8 value corresponds to an ASCII whitespace character
#[inline]
fn isspace(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}
