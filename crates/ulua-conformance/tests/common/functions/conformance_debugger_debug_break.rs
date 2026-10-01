use core::sync::atomic::Ordering;

use ulua_vm::records::{lua_debug::LuaDebug, lua_state::LuaState};

use crate::common::{
  functions::safe_api::{brk, debugtrace_raw},
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_debug_break(
  l: *mut LuaState,
  _ar: *mut LuaDebug,
) {
  // 命中计数是 safe 原子量自增（奇数次命中请求打断，与 cpp 的 static 计数等价）。
  let breakhits = CONFORMANCE_DEBUGGER_STATE
    .breakhits
    .fetch_add(1, Ordering::SeqCst)
    + 1;

  // `lua_debugtrace` 只读栈打印调用栈诊断，不改动栈；cpp 同位置对返回值取 void。
  debugtrace_raw(l);

  if breakhits % 2 == 1 {
    // 请求在下一个安全点打断执行。
    brk(l);
  }
}
