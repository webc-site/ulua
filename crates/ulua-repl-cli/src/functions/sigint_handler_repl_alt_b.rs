#![cfg(target_os = "windows")]

use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::functions::lua_callbacks::lua_callbacks;

use crate::functions::sigint_callback::{REPL_STATE, sigint_callback};

// Windows console control event for Ctrl-C (CTRL_C_EVENT).
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
pub unsafe extern "C-unwind" fn sigint_handler(signal: u32) -> c_int {
  unsafe {
    let repl_state = REPL_STATE.load(Ordering::SeqCst);
    if signal == CTRL_C_EVENT && !repl_state.is_null() {
      (*lua_callbacks(repl_state)).interrupt = Some(sigint_callback);
    }
    1 // TRUE
  }
}
