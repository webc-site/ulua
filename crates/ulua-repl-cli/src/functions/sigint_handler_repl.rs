use core::sync::atomic::Ordering;

use ulua_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{REPL_STATE, sigint_callback};

// Body of Repl.cpp's `sigintHandler` (both the POSIX and the Windows/
// `CTRL_C_EVENT` variant share it verbatim): arm the interrupt callback so
// the VM raises "Execution interrupted" at the next safe point. The OS-level
// entry forms — `unsafe extern "C-unwind"` signatures, the signum comparison
// and the Windows TRUE-return protocol — are handled by signal-hook-registry,
// which invokes this only for the one signal it was registered for.
///
/// 仅做 `REPL_STATE` 原子读取+判空，命中时写 `lua_callbacks` 的 interrupt 槽：
/// 无锁、无堆、无 panic，属 async-signal-safe 子集，只应从信号上下文（或
/// `REPL_STATE` 为有效/null 的进程早期路径）进入。状态地址仅在 run_repl 存活
/// 窗口内发布、close 前置回 null，故判空后的写目标对象存活；单槽指针写与 VM 端
/// （sigint_callback 的清除/重置）之间是 cpp oracle 同款的非原子竞态上界，
/// 行为是把中断再挂回一次或漏挂一拍，不产生内存不安全。
pub(crate) fn arm_interrupt() {
  // Safety: 见上——原子读 + 判空后才解引用，指针为 null 时不触碰任何内存。
  unsafe {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if !repl_state.is_null() {
      (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
  }
}
