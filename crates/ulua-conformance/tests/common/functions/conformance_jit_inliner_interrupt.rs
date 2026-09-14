use core::{
  ffi::c_int,
  sync::atomic::{AtomicI32, Ordering},
};

use ulua_vm::{luaL_error, records::lua_state::lua_State};

/// 中断计数器，对应 C++ JitInliner 测试中的 `static int index`
pub static JIT_INLINER_INDEX: AtomicI32 = AtomicI32::new(0);

/// 达到该次数即报超时（对应 C++ 的 1'000）
const TIMEOUT_HITS: i32 = 1_000;

/// # Safety
///
/// 指针参数须为有效的 `lua_State`，由 VM 经回调契约调用。
///
/// 内联回归测试包含死循环用例，靠 interrupt 兜底，达阈值即报 "timeout"。
pub unsafe extern "C-unwind" fn conformance_jit_inliner_interrupt(l: *mut lua_State, gc: c_int) {
  unsafe {
    // gc >= 0 表示 GC 步进，不计数
    if gc >= 0 {
      return;
    }

    if JIT_INLINER_INDEX.fetch_add(1, Ordering::SeqCst) + 1 >= TIMEOUT_HITS {
      luaL_error!(l, "timeout");
    }
  }
}
