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
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`ptr` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    if nsize == 0 {
      c_free(ptr.cast());
      // FFI: c-API 要求 NULL
      null_mut()
    } else {
      if !BLOCKABLE_REALLOC_ALLOWED.load(Ordering::Relaxed) {
        // FFI: c-API 要求 NULL
        return null_mut();
      }
      c_realloc(ptr.cast(), nsize).cast()
    }
  }
}
