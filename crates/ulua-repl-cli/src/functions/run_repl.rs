use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread},
  records::lua_state_guard::LuaStateGuard,
};

use crate::functions::{run_repl_impl::run_repl_impl, setup_state::setup_state, sigint_setup};

/// # Safety
///
/// Modifies global signal handling and manages a Lua VM state. Must only be run
/// in single-threaded REPL mode.
// Faithful port of `runRepl` from Repl.cpp: create a fresh state, set it up,
// arm Ctrl-C handling, sandbox the thread and run the interactive loop.
pub(crate) unsafe fn run_repl() {
  // lua_l_newstate 是安全封装（OOM 返回 null 的边界与 cpp 相同，下方各消费点
  // 契约同样覆盖）；守卫在作用域退出时 lua_close（panic/unwind 同样生效）。
  // cpp Repl.cpp:553 `unique_ptr<lua_State, void (*)(lua_State*)>`
  // 返回的 C-ABI 状态句柄由守卫持有，本帧只做别名传递，不做任何算术。
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  // Safety: REPL 进程入口路径（repl_main 保证单线程且无其它驱动者），l 为上一行
  // 新分配的状态、本帧存活，setup_state 的 /// # Safety 据此成立。
  unsafe { setup_state(l) };

  // setup Ctrl+C handling: replState = l; signal(SIGINT, sigintHandler);
  // 状态发布与 libc handler 注册（含与信号上下文交换的 null 协议值）收在
  // sigint_setup 门面里，这里只保留调用时机与存活契约。
  // Safety: l 为本帧新建、在整个交互式循环期间存活且单线程驱动（install 的
  // /// # Safety 前提）；循环结束后由下方 withdraw 先摘状态再 close。
  unsafe { sigint_setup::install(l) };

  // Safety: l 为本帧存活的刚建状态。
  unsafe { lua_l_sandboxthread(l) };
  // Safety: l 在整个交互式循环期间有效且单线程驱动（run_repl_impl 的 /// # Safety）。
  unsafe { run_repl_impl(l) };

  // cpp 只在 unique_ptr 析构处关闭；这里先把全局信号处理引用的状态摘掉，
  // 再由守卫在作用域退出时 lua_close（panic/unwind 路径同样生效）。
  sigint_setup::withdraw();
}
