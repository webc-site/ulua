/// cpp `LUAU_ASSERT(cond)` / `LUAU_ASSERT(cond, msg)` 对应：断言失败时经
/// `assert_call_handler` 上报，并按其返回值决定是否断点——host 注入的
/// `AssertHandler` 返回 0 即表示"已接管"，此时不再 debugbreak。
/// 仅在 `LUAU_ASSERTENABLED`（debug 或 `luau_assert` feature）下生效。
///
/// cpp 原型（`Common/include/Luau/Common.h:69`）：
/// `(void)(!!(expr) || (assertCallHandler(#expr, ...) && (LUAU_DEBUGBREAK(), 0)))`
#[macro_export]
macro_rules! LUAU_ASSERT {
  ($expr:expr $(, $msg:expr)?) => {
    if $crate::macros::luau_assert::LUAU_ASSERTENABLED {
      if !($expr)
        && $crate::functions::assert_call_handler::assert_fail(
          concat!(stringify!($expr) $(, " : ", stringify!($msg))?, "\0"),
          concat!(file!(), "\0"),
          line!() as i32,
        )
      {
        $crate::LUAU_DEBUGBREAK!();
      }
    }
  };
}

pub const LUAU_ASSERTENABLED: bool = cfg!(debug_assertions);

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
    $crate::macros::luau_assert::luau_debug_break();
  };
}

/// cpp `LUAU_UNREACHABLE()`：向编译器声明此点不可达，以换取更优代码路径。
///
/// # Safety
/// 调用方必须保证该控制流点在运行期确实不可达；否则 `unreachable_unchecked`
/// 是 UB（优化器可据此删除其后的所有代码）。仅在「前一分支已由 `LUAU_ASSERT!`
/// 覆盖 / 枚举分支穷尽但编译器无法证明」处使用。
#[macro_export]
macro_rules! LUAU_UNREACHABLE {
  () => {
    // Safety: 前置条件即本宏的 `# Safety` —— 由调用方保证此处不可达。
    unsafe {
      core::hint::unreachable_unchecked();
    }
  };
}

#[macro_export]
macro_rules! LUAU_UNLIKELY {
  ($x:expr) => {
    $x
  };
}

pub use LUAU_ASSERT;
pub use LUAU_DEBUGBREAK;
pub use LUAU_UNLIKELY;
pub use LUAU_UNREACHABLE;
