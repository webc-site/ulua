//! 进程级 Ctrl-C 处理的安装/撤回门面（review.md §2「`unsafe` 与 null 哨兵只留在
//! 带契约的收口点」）。
//!
//! `run_repl` 只需要两句语义：「把当前 REPL 状态登记为活动状态并挂上信号处理函数」
//! 与「循环结束后摘掉」。与 async-signal handler 交换的 `null` 协议值收在本模块内；
//! OS 级注册委托 signal-hook-registry（POSIX 走 `sigaction` + `SA_RESTART`，与
//! libc `signal()` 的 BSD 语义一致；Windows 走 CRT `signal()`，Ctrl+C 事件由
//! CRT 转成 SIGINT 送达），本 crate 不再出现 `unsafe extern` 声明、
//! kernel32 分支与裸 `null_mut()`。

use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};
use std::sync::Once;

use ulua_vm::records::lua_state::LuaState;

use crate::functions::{sigint_callback::REPL_STATE, sigint_handler_repl::arm_interrupt};

/// 信号号 `SIGINT`：POSIX 信号号为 2，Windows CRT（signal-hook-registry 内部
/// 调用的 `signal()`）同值，两平台共用一个常量。
const SIGINT: c_int = 2;

/// OS 级 handler 全进程只注册一次：`withdraw` 不摘 OS handler（handler 判空即
/// no-op，与 cpp 只换 `replState` 的做法一致），重复 `install` 若再注册会让
/// 每次 Ctrl-C 多跑一遍已登记的 handler。
static REGISTER_ONCE: Once = Once::new();

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
// DELIBERATE DEVIATION（review.md §9.3，平台 FFI 例外）：先发布 REPL 状态（null 协议值
// 见 REPL_STATE）再向 signal-hook-registry 注册进程级 SIGINT handler；该状态句柄会被
// 异步信号上下文解引用，故 unsafe 契约要求其存活覆盖整个注册期。
pub(crate) unsafe fn install(l: *mut LuaState) {
  // 发布的恒为非空活动状态（只有 `withdraw` 落回 null 协议值）：AtomicPtr 是主线程
  // 与 async-signal handler 之间唯一可安全交换的载体（信号上下文禁锁、禁堆原语），
  // 故「非活动」只能用 null 表示，该哨兵只在本门面读写。
  // Safety: 契约保证 l 指向存活状态；store 为原子写，不需要额外前置条件。
  REPL_STATE.store(l, Ordering::SeqCst);

  REGISTER_ONCE.call_once(|| {
    // `register` 现收编为安全 fn（唯一 unsafe 是 registry 边界，已下沉体内窄块）；
    // 状态已先行发布（call_once 前同步 store），register 的调用序契约成立。
    register();
  });
}

/// 摘掉活动状态（cpp `replState = nullptr`）。`null` 作为「REPL 非活动」协议值只在
/// 本门面出现，业务侧不再手写 `null_mut()`。
///
/// 安全函数：单槽原子存，无任何解引用。调用时机由 [`install`] 的契约约束——必须在
/// 状态被 `lua_close` 之前（`LuaStateGuard` 的作用域退出前）撤回。
pub(crate) fn withdraw() {
  // FFI: 平台 signal 上下文要求 NULL —— 「REPL 非活动」只能以 null 协议值表示
  // （见 REPL_STATE 的 DELIBERATE DEVIATION：AtomicPtr 是信号上下文唯一可安全交换
  // 的载体，无法承载 Option）；该 null 只在本门面读写，业务侧不出现裸 null_mut。
  REPL_STATE.store(null_mut(), Ordering::SeqCst);
}

/// 调用序契约（正确性，非内存安全）：仅在单线程 REPL 启动路径且状态先行发布后调用；
/// `arm_interrupt` 必须只做原子读 + 判空 + 单槽写（async-signal-safe 子集，
/// 契约见其文档），且永不 panic（registry 的 dispatch 发生在信号上下文）。
/// 无裸指针形参，唯一的真实不安全为 registry 的 OS 级注册调用，下沉为下方窄 `unsafe`
/// 块（review.md §2 诚实降级为安全 `fn`）。
// DELIBERATE DEVIATION（review.md §9.3，平台 FFI 例外）：委托 signal-hook-registry
// 完成 OS 级 sigaction/CRT signal 注册（POSIX/Windows 差异与裸 `unsafe extern`
// 声明均落在该库，本 crate 不再出现平台分支）；回调须 async-signal-safe。
fn register() {
  // registry 用 handler 数组取代单一 OS handler，注册后常驻进程（与原先
  // signal() 装上后直到进程退出都不摘的行为一致）；注册失败理论上不可能
  // （SIGINT 合法且不在 FORBIDDEN 表内），与原实现忽略 signal() 返回值
  // 同样静默略过。
  // SAFETY: `arm_interrupt` 是 async-signal-safe 的 `Fn()`（原子读 + 判空 +
  // 单槽写，无锁无堆无 panic），满足 registry::register 对回调的全部契约。
  let _ = unsafe { signal_hook_registry::register(SIGINT, arm_interrupt) };
}
