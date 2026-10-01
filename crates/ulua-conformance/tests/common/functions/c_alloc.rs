//! libc `free`/`realloc` 薄封装：conformance 各测试 allocator 共用一份
//! extern 声明（原先散落多处）。

use core::ffi::c_void;

unsafe extern "C" {
  fn free(ptr: *mut c_void);
  fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

/// # Safety
/// `ptr` 须为分配器返回的有效指针或 null（`free` 契约）。
pub unsafe fn c_free(ptr: *mut c_void) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`ptr` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { free(ptr) }
}

/// # Safety
/// `ptr` 须为分配器返回的有效指针或 null；`size` 为新大小。
pub unsafe fn c_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`ptr` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe { realloc(ptr, size) }
}
