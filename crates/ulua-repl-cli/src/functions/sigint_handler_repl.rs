use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{REPL_STATE, sigint_callback};

#[cfg(not(target_os = "windows"))]
const SIGINT: c_int = 2;

// Unix variant of Repl.cpp's `sigintHandler`:
//
//     static void sigintHandler(int signum)
//     {
//         if (signum == SIGINT && replState)
//             lua_callbacks(replState)->interrupt = &sigintCallback;
//     }
//
// Installed with `signal(SIGINT, sigintHandler)`; it merely arms the interrupt
// callback so the VM raises "Execution interrupted" at the next safe point.
/// # Safety
///
/// Must only be installed as a signal handler or called when `REPL_STATE` points
/// to a valid or null `lua_State`.
#[cfg(not(target_os = "windows"))]
pub(crate) unsafe extern "C-unwind" fn sigint_handler(signum: c_int) {
  // Safety: 信号处理器（OS 在任意时刻以本进程线程上下文调用）：仅做 REPL_STATE 原子读取+判空，命中时写 lua_callbacks 数组的 interrupt 槽。REPL_STATE 由 run_repl 仅在状态存活窗口内发布、close 前置回 null，故判空后的写目标对象存活；单槽指针写与 VM 端（sigint_callback 的清除/重置）之间是 cpp oracle 同款的非原子竞态上界，行为是把中断再挂回一次或漏挂一拍，不产生内存不安全。
  unsafe {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signum == SIGINT && !repl_state.is_null() {
      (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
  }
}

#[cfg(target_os = "windows")]
const CTRL_C_EVENT: u32 = 0;

// Windows variant of Repl.cpp's `sigintHandler`:
//
//     BOOL WINAPI sigintHandler(DWORD signal)
//     {
//         if (signal == CTRL_C_EVENT && replState)
//             lua_callbacks(replState)->interrupt = &sigintCallback;
//         return TRUE;
//     }
//
// Registered via `SetConsoleCtrlHandler`; returning TRUE (1) tells Windows the
// event was handled. Arms the same interrupt callback as the POSIX variant.
/// # Safety
///
/// Must only be registered as a console control handler or called when `REPL_STATE`
/// points to a valid or null `lua_State`.
#[cfg(target_os = "windows")]
pub(crate) unsafe extern "C-unwind" fn sigint_handler_windows(signal: u32) -> c_int {
  // Safety: Windows SetConsoleCtrlHandler 路径同 POSIX 变体：原子读 REPL_STATE+判空，写 lua_callbacks 的 interrupt 槽；状态地址仅在 run_repl 存活窗口发布，返回 TRUE 告知事件已处理。
  unsafe {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signal == CTRL_C_EVENT && !repl_state.is_null() {
      (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
    1 // TRUE
  }
}
