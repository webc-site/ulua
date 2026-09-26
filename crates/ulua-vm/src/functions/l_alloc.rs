use core::{ffi::c_void, ptr::null_mut};

/// # Safety
/// `ud` 为 `lua_newstate` 传入的分配器用户指针（可空）；`ptr` 为 null 或本分配器此前返回的块，
/// `osize` 为 0 表示纯分配、否则旧块可读。
pub unsafe extern "C-unwind" fn l_alloc(
  ud: *mut c_void,
  ptr: *mut u8,
  osize: usize,
  nsize: usize,
) -> *mut u8 {
  let _ = ud;
  let _ = osize;

  // Safety: free(NULL) 为标准 no-op，realloc(NULL, n) 等价 malloc(n)，无需判空分支；
  // 契约保证 ptr 为 null 或本分配器块，nsize==0 时归还并回 null，否则由 realloc 归还新块
  unsafe {
    if nsize == 0 {
      free(ptr.cast());
      null_mut()
    } else {
      realloc(ptr.cast(), nsize).cast()
    }
  }
}

// The C allocator surface. On native targets these `extern "C"` symbols bind
// the platform libc. On `wasm32-unknown-unknown` there is no libc, so they
// resolve at link time to `ulua_common::wasm_libc`'s size-prefixed allocator
// (backed by Rust's global allocator) — the VM allocates real memory in the
// browser.
// libc 符号签名对齐 C ABI，extern 声明保留 c_void
unsafe extern "C" {
  fn free(ptr: *mut c_void);
  fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}
