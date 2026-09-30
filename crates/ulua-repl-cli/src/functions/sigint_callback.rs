use core::{ffi::c_int, ptr::null_mut, sync::atomic::AtomicPtr};

use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_rawcheckstack::lua_rawcheckstack},
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

// `replState` from Repl.cpp: the REPL's LuaState, used by the OS signal handler
// to arm the interrupt callback. Stored atomically so the async-signal handler
// (sigintHandler) can read it safely.
// DELIBERATE DEVIATION（review.md §9.3，平台 FFI 例外）：null 即 cpp `static
// LuaState* replState` 的「REPL 非活动」协议哨兵。AtomicPtr 是与 async-signal
// handler 唯一可安全交换的载体（信号上下文禁锁、禁堆，无法承载 `Option`），故此处
// 保留裸指针 + null 表示缺席，非纯 Rust 逻辑；跨界判空只在 sigint_setup 门面与本
// 模块内发生。
pub static REPL_STATE: AtomicPtr<LuaState> = AtomicPtr::new(null_mut());

/// # Safety
///
/// `l` must be a valid, active pointer to a `LuaState`.
// Ctrl-C handling. Matches the `interrupt` callback ABI on LuaCallbacks.
// DELIBERATE DEVIATION（review.md §9.3）：以 `extern "C-unwind"` 形态写入 VM
// `LuaCallbacks::interrupt` 槽并在 safepoint 处按 C ABI 被回调，裸 `*mut LuaState`
// 与 `c_int` 形参系槽位 ABI 契约要求；内部 `lua_callbacks`/`lua_rawcheckstack`/
// `luaL_error` 为 ulua-vm c-API 边界。
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
