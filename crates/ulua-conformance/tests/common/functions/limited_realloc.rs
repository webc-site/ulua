use core::{ffi::c_void, ptr::null_mut};

use crate::common::functions::c_alloc::{c_free, c_realloc};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn limited_realloc(
  _ud: *mut c_void,
  ptr: *mut u8,
  _osize: usize,
  nsize: usize,
) -> *mut u8 {
  if nsize == 0 {
    unsafe { c_free(ptr.cast()) };
    null_mut()
  } else if nsize > 8 * 1024 * 1024 {
    // 测试用：超大分配返回 null，用于构造内存分配失败错误
    null_mut()
  } else {
    unsafe { c_realloc(ptr.cast(), nsize).cast() }
  }
}
