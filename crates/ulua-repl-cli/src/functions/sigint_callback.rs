use core::{ffi::c_int, ptr::null_mut, sync::atomic::AtomicPtr};

use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_rawcheckstack::lua_rawcheckstack},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

// `replState` from Repl.cpp: the REPL's LuaState, used by the OS signal handler
// to arm the interrupt callback. Stored atomically so the async-signal handler
// (sigintHandler) can read it safely.
// 保留空指针：null 即 cpp `static LuaState* replState` 的「REPL 非活动」
// 哨兵，信号 handler 仅做原子读+判 null（async-signal 安全子集）；
// AtomicPtr 无法承载 Option，跨界原语要求裸指针表示。
pub static REPL_STATE: AtomicPtr<LuaState> = AtomicPtr::new(null_mut());

/// # Safety
///
/// `l` must be a valid, active pointer to a `LuaState`.
// Ctrl-C handling. Matches the `interrupt` callback ABI on LuaCallbacks.
pub(crate) unsafe extern "C-unwind" fn sigint_callback(l: *mut LuaState, gc: c_int) {
  if gc >= 0 {
    return;
  }

  // Safety: l 是 VM 在安全点调用 interrupt 回调时传入的当前线程有效状态（Lua/C API
  // 回调约定）；lua_callbacks(l) 返回该状态固定区内的回调数组（非空），本行仅写
  // interrupt 槽为 None（先行摘除防重入），发生在 VM 线程自身栈上。
  unsafe { (*lua_callbacks(l)).interrupt = None };

  // Safety: l 存活；checkstack 预留错误串槽位后 luaL_error 经 VM 错误机制发散。
  unsafe {
    lua_rawcheckstack(l, 1); // reserve space for error string
    luaL_error!(l, "Execution interrupted");
  }
}
