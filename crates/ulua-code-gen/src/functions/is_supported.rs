use core::mem::size_of;

#[cfg(not(windows))]
use ulua_vm::macros::lua_use_longjmp::LUA_USE_LONGJMP;
use ulua_vm::{
  macros::lua_extra_size::LUA_EXTRA_SIZE,
  records::{lua_node::LuaNode, lua_t_value::TValue},
};

use crate::functions::is_unwind_supported::is_unwind_supported;

// 与 cpp CodeGen/src/CodeGen.cpp isSupported() 逐条对齐。
// 目标架构分支沿用 macros/codegen_target.rs 的
// 谓词定义：#[cfg] 只接受真实 cfg 谓词，不能引用 const 布尔。

/// 布局门槛的编译期合取：`LUA_EXTRA_SIZE` 为 const、`size_of` 为 const fn，三项
/// 均不依赖机器状态（cfg 无关的编译期事实），按 review §6 收口为单个 `const bool`，
/// 支持布局下这些判断在运行期零指令；不支持布局的行为与逐条 `return false` 等价。
const LAYOUT_SUPPORTED: bool =
  LUA_EXTRA_SIZE == 1 && size_of::<TValue>() == 16 && size_of::<LuaNode>() == 32;

pub fn is_supported() -> bool {
  if !LAYOUT_SUPPORTED {
    return false;
  }

  // Windows CRT 在 longjmp 中使用栈展开，必须提供 unwind 数据；
  // 其余平台仅 C++ EH 需要。
  // cpp: #if defined(_WIN32) ... #else if (!LUA_USE_LONGJMP && !isUnwindSupported())
  #[cfg(windows)]
  if !is_unwind_supported() {
    return false;
  }
  #[cfg(not(windows))]
  if LUA_USE_LONGJMP == 0 && !is_unwind_supported() {
    return false;
  }

  // cpp: #if defined(CODEGEN_TARGET_X64)（谓词见 macros/codegen_target.rs）
  // 要求 AVX1（VEX 编码 XMM 指令）；CPUID EAX=1 的 ECX bit 28，
  // 该位同时隐含 SSE4.1（ROUNDSD 依赖）。
  // https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
  #[cfg(target_arch = "x86_64")]
  {
    use core::arch::x86_64::__cpuid;
    return __cpuid(1).ecx & (1 << 28) != 0;
  }

  #[cfg(target_arch = "x86")]
  {
    use core::arch::x86::__cpuid;
    return __cpuid(1).ecx & (1 << 28) != 0;
  }

  // cpp: #elif defined(CODEGEN_TARGET_A64) return true;
  #[cfg(all(target_arch = "aarch64", not(target_os = "windows")))]
  return true;

  // cpp: #else return false;
  #[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    all(target_arch = "aarch64", not(target_os = "windows"))
  )))]
  return false;
}
