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
  if nsize == 0 {
    if !ptr.is_null() && osize != 0 {
      // Safety: VM 分配器契约保证 `ptr` 由本例程以 (osize, ALIGNMENT) 分配、尚未释放。
      unsafe {
        dealloc(
          ptr,
          Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout"),
        );
      }
    }
    // FFI: c-API 要求 NULL
    return null_mut();
  }

  let new_layout = Layout::from_size_align(nsize, ALIGNMENT).expect("valid new layout");
  if ptr.is_null() || osize == 0 {
    // Safety: `nsize` 非零（上面已分支返回），新分配无既有块约束；返回裸块由调用方接管。
    unsafe { alloc(new_layout) }
  } else {
    let old_layout = Layout::from_size_align(osize, ALIGNMENT).expect("valid old layout");
    // Safety: VM 分配器契约保证 `ptr` 由本例程以 `old_layout` 的参数分配、尚未释放，
    // `nsize` 非零。
    unsafe { realloc(ptr, old_layout, nsize) }
  }
}
