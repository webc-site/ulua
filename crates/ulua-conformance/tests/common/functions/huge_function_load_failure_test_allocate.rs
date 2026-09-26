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
  if nsize == 0 {
    // Safety: VM 分配器契约保证 `ptr` 为 null 或由本例程交回的块，`c_free` 承接释放。
    unsafe { c_free(ptr.cast()) };
    // FFI: c-API 要求 NULL
    return null_mut();
  }

  // 计数与阈值比较都是 safe 原子量：命中「第 N 次大分配」时按要求返回失败。
  if nsize > 32768
    && HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.load(Ordering::SeqCst)
      == HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_TO_FAIL.load(Ordering::SeqCst)
  {
    // FFI: c-API 要求 NULL
    return null_mut();
  }

  if nsize > 32768 {
    HUGE_FUNCTION_LOAD_FAILURE_LARGE_ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
  }

  // Safety: `ptr` 为 VM 交回的既有块（null 表示首次分配），`nsize` 非零（上面已分支返回）。
  unsafe { c_realloc(ptr.cast(), nsize).cast() }
}
