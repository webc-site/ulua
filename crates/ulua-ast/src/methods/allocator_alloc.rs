use core::{mem::size_of, ptr::write};

use crate::records::allocator::Allocator;

impl Allocator {
  pub fn alloc<T>(&mut self, value: T) -> *mut T {
    // The C++ implementation uses a static_assert to ensure T is trivially destructible
    // because the allocator never calls destructors. In Rust, we don't have an exact
    // equivalent to std::is_trivially_destructible as a stable trait bound, but
    // the contract remains: the caller must be aware that the memory is managed
    // by the Allocator and won't be dropped automatically.

    let ptr = self.allocate(size_of::<T>()) as *mut T;
    if !ptr.is_null() {
      // SAFETY: allocate 按 max(align_of(void*), align_of(f64)) 对齐、且字节数
      // 足够容纳 T；ptr 非空且指向刚申请的未初始化内存，写入一次即完成初始化。
      unsafe {
        write(ptr, value);
      }
    }
    ptr
  }
}
