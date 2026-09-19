use core::{ffi::c_void, ptr::null_mut, sync::atomic::Ordering};

use crate::common::functions::{
  blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED,
  c_alloc::{c_free, c_realloc},
};

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
      c_free(ptr.cast());
      null_mut()
    } else {
      if !BLOCKABLE_REALLOC_ALLOWED.load(Ordering::Relaxed) {
        return null_mut();
      }
      c_realloc(ptr.cast(), nsize).cast()
    }
  }
}
