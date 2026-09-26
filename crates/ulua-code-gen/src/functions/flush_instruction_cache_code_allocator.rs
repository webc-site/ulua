#[cfg(target_os = "windows")]
use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn flush_instruction_cache(mem: *mut u8, size: usize) {
  #[cfg(target_os = "windows")]
  {
    use core::ffi::c_void;

    use windows_sys::Win32::System::{
      Diagnostics::Debug::FlushInstructionCache, Threading::GetCurrentProcess,
    };

    // Safety: mem/size 为分配器配对产出的映射区（非空、界内）；
    // FlushInstructionCache 为 Win32 C ABI，hProcess 取当前进程合法伪句柄。
    unsafe {
      if FlushInstructionCache(GetCurrentProcess(), mem as *const c_void, size) == 0 {
        CODEGEN_ASSERT!(false);
      }
    }
  }
  #[cfg(not(target_os = "windows"))]
  {
    flush_instruction_cache_mut(mem, size);
  }
}

pub fn flush_instruction_cache_mut(mem: *mut u8, size: usize) {
  #[cfg(target_arch = "wasm32")]
  {
    // Wasm/Emscripten 无指令缓存一致性问题，无需刷新
    let _ = (mem, size);
  }
  #[cfg(all(not(target_arch = "wasm32"), target_vendor = "apple"))]
  {
    use core::ffi::c_void;

    unsafe extern "C" {
      fn sys_icache_invalidate(start: *mut c_void, len: usize);
    }
    // Safety: mem/size 由调用方成对提供（页对齐代码区），sys_icache_invalidate 为 Apple C ABI，
    // start 非空、len 为该区内长度，仅做指令缓存失效、不改写内存内容，无别名风险。
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
    // x86 / x86_64 硬件保证指令缓存与数据缓存一致，
    // 写完代码后无需显式 flush。
    let _ = (mem, size);
  }
  #[cfg(all(
    not(target_arch = "wasm32"),
    not(target_vendor = "apple"),
    not(any(target_arch = "x86", target_arch = "x86_64"))
  ))]
  {
    // 其他架构（如 aarch64 Linux）需要显式 flush i-cache。
    // `__clear_cache` 是 GCC/Clang builtin（此前经由
    // nightly-only 的 `llvm.clear_cache` intrinsic 到达，新版 stable
    // rustc 已拒绝该 intrinsic）；这些目标上由 compiler-builtins 提供。
    unsafe extern "C" {
      fn __clear_cache(begin: *mut core::ffi::c_char, end: *mut core::ffi::c_char);
    }
    // Safety: mem/size 为分配器配对产出的映射区；[mem, mem+size) 为合法可寻址字节区间，
    // __clear_cache 为编译器内建 C ABI，begin<=end，仅做缓存一致性操作、不改写内容。
    unsafe {
      __clear_cache(
        mem as *mut core::ffi::c_char,
        mem.add(size) as *mut core::ffi::c_char,
      );
    }
  }
}
