use core::mem::size_of;

use ulua_vm::{
  macros::{lua_extra_size::LUA_EXTRA_SIZE, lua_use_longjmp::LUA_USE_LONGJMP},
  records::{lua_node::LuaNode, lua_t_value::TValue},
};

use crate::functions::is_unwind_supported::is_unwind_supported;

// NOTE: This function is native-only in the original codegen. It depends on
// platform ABI assumptions (TValue/LuaNode layout) and CPU feature detection.
pub fn is_supported() -> bool {
  if LUA_EXTRA_SIZE != 1 {
    return false;
  }

  // The JIT emits machine code with hardcoded structure sizes/offsets, so the
  // runtime ABI layout must match what the code generator assumes. C++
  // (CodeGen/src/CodeGen.cpp) bails out of native codegen if these don't hold:
  //   if (sizeof(TValue) != 16) return false;
  //   if (sizeof(LuaNode) != 32) return false;
  if size_of::<TValue>() != 16 {
    return false;
  }

  if size_of::<LuaNode>() != 32 {
    return false;
  }

  #[cfg(not(windows))]
  {
    if LUA_USE_LONGJMP == 0 && !is_unwind_supported() {
      return false;
    }
  }
  #[cfg(windows)]
  {
    if !is_unwind_supported() {
      return false;
    }
  }

  #[cfg(CODEGEN_TARGET_X64)]
  {
    // CPU feature check for AVX1/VEX encoded XMM ops and ROUNDSD via SSE4.1.
    // Use the same CPUID leaf as the C++ code: EAX=1.
    //
    // We use is_x86_feature_detected! which is supported on wasm32 for
    // non-native targets behind cfg; this function is native-only so it is
    // fine to rely on x86 intrinsics when compiling for x86.
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
      use core::arch::x86::__cpuid;

      let cpuinfo = unsafe { __cpuid(1) };
      // https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
      // cpuinfo.ecx holds feature bits. Bit 28 == AVX1.
      if (cpuinfo.ecx & (1 << 28)) == 0 {
        return false;
      }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    {
      return false;
    }

    return true;
  }

  #[cfg(CODEGEN_TARGET_A64)]
  {
    return true;
  }

  false
}
