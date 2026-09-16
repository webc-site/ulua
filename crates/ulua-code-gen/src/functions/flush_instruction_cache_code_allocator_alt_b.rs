use core::ffi::c_void;
pub fn flush_instruction_cache_mut(mem: *mut u8, size: usize) {
  #[cfg(target_arch = "wasm32")]
  {
    // No-op for Emscripten/Wasm
  }
  #[cfg(all(not(target_arch = "wasm32"), target_vendor = "apple"))]
  {
    unsafe extern "C" {
      fn sys_icache_invalidate(start: *mut c_void, len: usize);
    }
    unsafe {
      sys_icache_invalidate(mem as *mut c_void, size);
    }
  }
  #[cfg(all(
    not(target_arch = "wasm32"),
    not(target_vendor = "apple"),
    any(target_arch = "x86", target_arch = "x86_64")
  ))]
  {
    // x86 / x86_64 keep the instruction and data caches coherent in hardware,
    // so no explicit flush is needed after writing code.
    let _ = (mem, size);
  }
  #[cfg(all(
    not(target_arch = "wasm32"),
    not(target_vendor = "apple"),
    not(any(target_arch = "x86", target_arch = "x86_64"))
  ))]
  {
    // Other architectures (e.g. aarch64 Linux) need an explicit i-cache flush.
    // `__clear_cache` is the GCC/Clang builtin (formerly reached here via the
    // nightly-only `llvm.clear_cache` intrinsic, which newer stable rustc
    // rejects); it is provided by compiler-builtins on these targets.
    unsafe extern "C" {
      fn __clear_cache(begin: *mut core::ffi::c_char, end: *mut core::ffi::c_char);
    }
    unsafe {
      __clear_cache(
        mem as *mut core::ffi::c_char,
        mem.add(size) as *mut core::ffi::c_char,
      );
    }
  }
}
