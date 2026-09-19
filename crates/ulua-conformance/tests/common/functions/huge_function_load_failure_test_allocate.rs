use core::{
  ffi::c_void,
  ptr::null_mut,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::common::functions::c_alloc::{c_free, c_realloc};

pub static HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL: AtomicUsize = AtomicUsize::new(0);
pub static HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn huge_function_load_failure_test_allocate(
  _ud: *mut c_void,
  ptr: *mut u8,
  _osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe {
    if nsize == 0 {
      c_free(ptr.cast());
      null_mut()
    } else if nsize > 32768
      && HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.load(Ordering::SeqCst)
        == HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL.load(Ordering::SeqCst)
    {
      null_mut()
    } else {
      if nsize > 32768 {
        HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
      }
      c_realloc(ptr.cast(), nsize).cast()
    }
  }
}
