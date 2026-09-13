use core::{ffi::c_void, ptr::null_mut};

use crate::common::functions::blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn blockable_realloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  _osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe {
    if nsize == 0 {
      libc::free(ptr as *mut c_void);
      null_mut()
    } else {
      if !BLOCKABLE_REALLOC_ALLOWED {
        return null_mut();
      }
      libc::realloc(ptr as *mut c_void, nsize) as *mut u8
    }
  }
}

mod libc {
  use core::ffi::c_void;

  unsafe extern "C" {
    pub fn free(ptr: *mut c_void);
    pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
  }
}
