use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{REPL_STATE, sigint_callback};

// SIGINT value on POSIX systems.
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
pub unsafe extern "C-unwind" fn sigint_handler(signum: c_int) {
  unsafe {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signum == SIGINT && !repl_state.is_null() {
      (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
  }
}
