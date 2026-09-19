use core::{ffi::c_void, ptr::null_mut};

use crate::common::functions::c_alloc::{c_free, c_realloc};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn alloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  _osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe {
    if nsize == 0 {
      c_free(ptr.cast());
      null_mut()
    } else {
      c_realloc(ptr.cast(), nsize).cast()
    }
  }
}
