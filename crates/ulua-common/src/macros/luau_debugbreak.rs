#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use core::arch::asm;
#[cfg(target_arch = "wasm32")]
use core::arch::wasm32;

/// b28 裁定：必须保持 `pub`——`LUAU_DEBUGBREAK!`/`LUAU_ASSERT!` 宏体经
/// `$crate::…` 在下游 crate 展开消费（零消费点计数假阳性）。
#[inline]
pub fn luau_debug_break() {
  // x86/x86_64 共用 `int 3` 断点指令。
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  // Safety: `int 3` 是 x86 架构定义的断点陷阱指令，无内存操作数、不读写寄存器、
  // 仅触发调试异常；在无调试器接管时按平台语义产生 SIGTRAP，不破坏内存不变量。
  unsafe {
    asm!("int 3");
  }
  #[cfg(target_arch = "aarch64")]
  // Safety: `brk #0xf000` 是 AArch64 架构定义的软件断点指令，无操作数、不修改内存，
  // 仅触发调试异常；与 x86 分支同理，仅在 debug-break 语境调用，不违反任何 unsafe 前置。
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
