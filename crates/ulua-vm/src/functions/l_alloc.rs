use core::{ffi::c_void, ptr::null_mut};
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn l_alloc(
  ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  let _ = ud;
  let _ = osize;

  unsafe {
    // free(NULL) 为标准 no-op，realloc(NULL, n) 等价 malloc(n)，无需判空分支
    if nsize == 0 {
      libc_free(ptr);
      null_mut()
    } else {
      libc_realloc(ptr, nsize)
    }
  }
}

// The C allocator surface. On native targets these `extern "C"` symbols bind
// the platform libc. On `wasm32-unknown-unknown` there is no libc, so they
// resolve at link time to `ulua_common::wasm_libc`'s size-prefixed allocator
// (backed by Rust's global allocator) — the VM allocates real memory in the
// browser rather than the previous null-returning wasm stub.
// libc 符号签名对齐 C ABI，extern 声明保留 c_void
unsafe extern "C" {
  fn free(ptr: *mut c_void);
  fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

unsafe fn libc_free(ptr: *mut u8) {
  unsafe {
    free(ptr as *mut c_void);
  }
}

unsafe fn libc_realloc(ptr: *mut u8, nsize: usize) -> *mut u8 {
  unsafe { realloc(ptr as *mut c_void, nsize) as *mut u8 }
}
