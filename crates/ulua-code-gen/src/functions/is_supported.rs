use core::mem::size_of;

use ulua_vm::{
  macros::{lua_extra_size::LUA_EXTRA_SIZE, lua_use_longjmp::LUA_USE_LONGJMP},
  records::{lua_node::LuaNode, lua_t_value::TValue},
};

use crate::functions::is_unwind_supported::is_unwind_supported;

// 与 cpp CodeGen/src/CodeGen.cpp isSupported() 逐条对齐。
// 目标架构分支沿用 macros/codegen_target_x_64.rs、codegen_target_a_64.rs 的
// 谓词定义：#[cfg] 只接受真实 cfg 谓词，不能引用 const 布尔。
pub fn is_supported() -> bool {
  if LUA_EXTRA_SIZE != 1 {
    return false;
  }

  // cpp: if (sizeof(TValue) != 16) return false;
  if size_of::<TValue>() != 16 {
    return false;
  }

  // cpp: if (sizeof(LuaNode) != 32) return false;
  if size_of::<LuaNode>() != 32 {
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

  // cpp: #if defined(CODEGEN_TARGET_X64)（谓词见 macros/codegen_target_x_64.rs）
  // 要求 AVX1（VEX 编码 XMM 指令）；CPUID EAX=1 的 ECX bit 28，
  // 该位同时隐含 SSE4.1（ROUNDSD 依赖）。
  // https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
  #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
  return unsafe { core::arch::x86::__cpuid(1) }.ecx & (1 << 28) != 0;

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
