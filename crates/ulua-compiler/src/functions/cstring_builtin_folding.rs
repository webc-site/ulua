use core::ffi::c_char;

use crate::{
  functions::cstring_builtin_folding_alt_b::cstring_c_char_usize, records::constant::Constant,
};

/// Creates a string constant from a byte slice.
#[inline]
pub fn cstring_slice(s: &[u8]) -> Constant {
  cstring_c_char_usize(s.as_ptr() as *const c_char, s.len())
}

/// Creates a string constant from a string slice.
#[inline]
pub fn cstring_str(s: &str) -> Constant {
  cstring_slice(s.as_bytes())
}
