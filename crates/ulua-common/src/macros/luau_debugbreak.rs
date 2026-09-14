#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use core::arch::asm;
#[cfg(target_arch = "wasm32")]
use core::arch::wasm32;

#[inline(always)]
pub fn luau_debug_break() {
  #[cfg(target_arch = "x86")]
  unsafe {
    asm!("int 3");
  }
  #[cfg(target_arch = "x86_64")]
  unsafe {
    asm!("int 3");
  }
  #[cfg(target_arch = "aarch64")]
  unsafe {
    asm!("brk #0xf000");
  }
  // wasm32::unreachable 是安全函数，无需 unsafe。
  #[cfg(target_arch = "wasm32")]
  wasm32::unreachable();
  #[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32"
  )))]
  {
    panic!("LUAU_DEBUGBREAK");
  }
}

#[macro_export]
macro_rules! LUAU_DEBUGBREAK {
  () => {
    $crate::macros::luau_debugbreak::luau_debug_break();
  };
}

pub use LUAU_DEBUGBREAK;
