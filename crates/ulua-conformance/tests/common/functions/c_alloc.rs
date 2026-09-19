//! libc `free`/`realloc` 薄封装：conformance 各测试 allocator 与
//! `luau_compile` 字节码缓冲释放共用一份 extern 声明（原先散落 5 处）。

use core::ffi::c_void;

unsafe extern "C" {
  fn free(ptr: *mut c_void);
  fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

/// # Safety
/// `ptr` 须为分配器返回的有效指针或 null（`free` 契约）。
pub unsafe fn c_free(ptr: *mut c_void) {
  unsafe { free(ptr) }
}

/// # Safety
/// `ptr` 须为分配器返回的有效指针或 null；`size` 为新大小。
pub unsafe fn c_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
  unsafe { realloc(ptr, size) }
}
