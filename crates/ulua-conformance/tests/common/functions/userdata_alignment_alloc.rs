use core::{ffi::c_void, ptr::null_mut};
use std::alloc::{Layout, alloc, dealloc, realloc};

const ALIGNMENT: usize = 16;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_alignment_alloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  unsafe {
    if nsize == 0 {
      if !ptr.is_null() && osize != 0 {
        dealloc(
          ptr,
          Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout"),
        );
      }
      return null_mut();
    }

    let new_layout = Layout::from_size_align(nsize, ALIGNMENT).expect("valid new layout");
    if ptr.is_null() || osize == 0 {
      alloc(new_layout)
    } else {
      let old_layout = Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout");
      realloc(ptr, old_layout, nsize)
    }
  }
}
