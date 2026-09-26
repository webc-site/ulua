//! 进程级 Ctrl-C 处理的安装/撤回门面（review.md §2「`unsafe` 与 null 哨兵只留在
//! 带契约的收口点」）。
//!
//! `run_repl` 只需要两句语义：「把当前 REPL 状态登记为活动状态并挂上信号处理函数」
//! 与「循环结束后摘掉」。libc `signal()` / kernel32 `SetConsoleCtrlHandler()` 的
//! 外部函数声明、以及与 async-signal handler 交换的 `null` 协议值都收在本模块内，
//! 业务侧因此不再出现裸 `null_mut()` 与 `unsafe extern` 声明。

use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};

use ulua_vm::records::lua_state::LuaState;

use crate::functions::sigint_callback::REPL_STATE;

/// POSIX 信号号 `SIGINT`（与 `sigint_handler_repl` 的判等常量同值；两处各自 cfg
/// 编译，不共享以免跨 cfg 泄漏符号）。
#[cfg(not(target_os = "windows"))]
const SIGINT: c_int = 2;

/// 登记活动状态并注册进程级 Ctrl-C 处理函数，对应 cpp `Repl.cpp` 的
/// `replState = l; signal(SIGINT, sigintHandler);`。
///
/// 顺序即契约：先发布状态、后注册 handler，保证 handler 一旦可能被触发，
/// `REPL_STATE` 已经指向有效状态（handler 只做原子读 + 判空）。
///
/// # Safety
///
/// `l` 必须指向在**整个 REPL 循环期间**存活的 `LuaState`，且调用方处于单线程驱动的
/// REPL 入口路径：信号 handler 由 OS 在任意异步时刻进入并解引用该状态，故其存活期
/// 必须覆盖注册期；本函数返回前发布的状态由 [`withdraw`] 在任何关闭路径之前摘掉。
pub(crate) unsafe fn install(l: *mut LuaState) {
  // 发布的恒为非空活动状态（只有 `withdraw` 落回 null 协议值）：AtomicPtr 是主线程
  // 与 async-signal handler 之间唯一可安全交换的载体（信号上下文禁锁、禁堆原语），
  // 故「非活动」只能用 null 表示，该哨兵只在本门面读写。
  // Safety: 契约保证 l 指向存活状态；store 为原子写，不需要额外前置条件。
  REPL_STATE.store(l, Ordering::SeqCst);

  // Safety: 状态已先行发布，注册本身只做一次 libc/kernel32 调用（见下两个 cfg 变体）。
  unsafe { register() };
}

/// 摘掉活动状态（cpp `replState = nullptr`）。`null` 作为「REPL 非活动」协议值只在
/// 本门面出现，业务侧不再手写 `null_mut()`。
///
/// 安全函数：单槽原子存，无任何解引用。调用时机由 [`install`] 的契约约束——必须在
/// 状态被 `lua_close` 之前（`LuaStateGuard` 的作用域退出前）撤回。
pub(crate) fn withdraw() {
  REPL_STATE.store(null_mut(), Ordering::SeqCst);
}

#[cfg(not(target_os = "windows"))]
/// # Safety
///
/// 注册进程级 SIGINT 处理函数，仅在单线程 REPL 启动路径调用；`sigint_handler` 必须是
/// 签名匹配的 `unsafe extern "C-unwind" fn(c_int)`，且调用前 `REPL_STATE` 已指向存活的
/// `LuaState`（信号异步进入 handler，依赖该全局的有效性与单线程驱动契约）。
unsafe fn register() {
  use core::ffi::c_void;

  use crate::functions::sigint_handler_repl::sigint_handler;
  // POSIX: signal(SIGINT, sigintHandler)
  unsafe extern "C" {
    fn signal(signum: c_int, handler: unsafe extern "C-unwind" fn(c_int)) -> *mut c_void;
  }
  // Safety: libc signal() 的 FFI 调用：SIGINT 为合法信号号，sigint_handler 是与处理函数签名匹配的 unsafe extern "C-unwind" fn(c_int)（指针可安全转成 sighandler_t），且 handler 只做原子读+单槽写入（async-signal 安全子集，契约见其 /// # Safety）。
  unsafe {
    signal(SIGINT, sigint_handler);
  }
}

#[cfg(target_os = "windows")]
/// # Safety
///
/// 注册进程级控制台 Ctrl 处理函数，仅在单线程 REPL 启动路径调用；`sigint_handler_windows` 必须是
/// 签名匹配的 `unsafe extern "C-unwind" fn(u32) -> c_int`，且调用前 `REPL_STATE` 已指向
/// 存活的 `LuaState`（回调跨线程进入 handler，依赖该全局有效性与单线程驱动契约）。
unsafe fn register() {
  use crate::functions::sigint_handler_repl::sigint_handler_windows;

  // Windows: SetConsoleCtrlHandler(sigintHandler, TRUE)
  unsafe extern "system" {
    fn SetConsoleCtrlHandler(
      handler: Option<unsafe extern "C-unwind" fn(u32) -> c_int>,
      add: c_int,
    ) -> c_int;
  }
  // Safety: kernel32 SetConsoleCtrlHandler 的 FFI 调用：handler 为签名匹配的
  // unsafe extern "C-unwind" fn（Option 包裹即合法注册形态），add=1 表 TRUE；
  // 注册前置条件（REPL_STATE 有效性）由本 fn 的 /// # Safety 契约承担。
  unsafe {
    SetConsoleCtrlHandler(Some(sigint_handler_windows), 1);
  }
}
