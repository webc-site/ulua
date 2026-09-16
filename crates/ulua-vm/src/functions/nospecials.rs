use core::ffi::{CStr, c_char, c_int};
const SPECIALS: &[u8] = b"^$*+?.([%-";

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn nospecials(p: *const c_char, l: usize) -> c_int {
  let mut upto: usize = 0;
  loop {
    unsafe {
      let current_p = p.add(upto);
      let s = CStr::from_ptr(current_p);
      let bytes = s.to_bytes();

      // Check if any character in the current null-terminated segment is a special character
      for &b in bytes {
        for &spec in SPECIALS {
          if b == spec {
            return 0;
          }
        }
      }

      upto += bytes.len() + 1; // Move past the segment and its null terminator
    }

    if upto > l {
      break;
    }
  }
  1
}
