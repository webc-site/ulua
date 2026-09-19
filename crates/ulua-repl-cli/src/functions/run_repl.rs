use core::{ffi::c_int, ptr::null_mut, sync::atomic::Ordering};

use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread,
  },
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
    let global_state = lua_l_newstate();
    let l: *mut lua_State = global_state;

    setup_state(l);

    // setup Ctrl+C handling: replState = l; signal(SIGINT, sigintHandler);
    REPL_STATE.store(l, Ordering::SeqCst);
    install_sigint_handler();

    lua_l_sandboxthread(l);
    run_repl_impl(l);

    // C++ wraps the state in a unique_ptr<lua_State, lua_close>; close it here.
    REPL_STATE.store(null_mut(), Ordering::SeqCst);
    lua_close(global_state);
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
