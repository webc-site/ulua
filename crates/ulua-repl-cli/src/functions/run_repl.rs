use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};

use ulua_cli_lib::records::lua_state_guard::LuaStateGuard;
use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread},
  type_aliases::lua_state::lua_State,
};

use crate::functions::{
  run_repl_impl::run_repl_impl, setup_state::setup_state, sigint_callback::REPL_STATE,
};

/// # Safety
///
/// Modifies global signal handling and manages a Lua VM state. Must only be run
/// in single-threaded REPL mode.
// Faithful port of `runRepl` from Repl.cpp: create a fresh state, set it up,
// arm Ctrl-C handling, sandbox the thread and run the interactive loop.
pub unsafe fn run_repl() {
  unsafe {
    // cpp Repl.cpp:553 `unique_ptr<lua_State, void (*)(lua_State*)>` —— 守卫负责关闭
    let global_state = LuaStateGuard(lua_l_newstate());
    let l: *mut lua_State = global_state.0;

    setup_state(l);

    // setup Ctrl+C handling: replState = l; signal(SIGINT, sigintHandler);
    REPL_STATE.store(l, Ordering::SeqCst);
    install_sigint_handler();

    lua_l_sandboxthread(l);
    run_repl_impl(l);

    // cpp 只在 unique_ptr 析构处关闭；这里先把全局信号处理引用的状态摘掉，
    // 再由守卫在作用域退出时 lua_close（panic/unwind 路径同样生效）。
    REPL_STATE.store(null_mut(), Ordering::SeqCst);
  }
}

#[cfg(not(target_os = "windows"))]
unsafe fn install_sigint_handler() {
  use core::ffi::c_void;

  use crate::functions::sigint_handler_repl::sigint_handler;
  // POSIX: signal(SIGINT, sigintHandler)
  const SIGINT: c_int = 2;
  unsafe extern "C" {
    fn signal(signum: c_int, handler: unsafe extern "C-unwind" fn(c_int)) -> *mut c_void;
  }
  unsafe {
    signal(SIGINT, sigint_handler);
  }
}

#[cfg(target_os = "windows")]
unsafe fn install_sigint_handler() {
  use crate::functions::sigint_handler_repl_alt_b::sigint_handler;

  // Windows: SetConsoleCtrlHandler(sigintHandler, TRUE)
  unsafe extern "system" {
    fn SetConsoleCtrlHandler(
      handler: Option<unsafe extern "C-unwind" fn(u32) -> c_int>,
      add: c_int,
    ) -> c_int;
  }
  unsafe {
    SetConsoleCtrlHandler(Some(sigint_handler), 1);
  }
}
