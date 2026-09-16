use core::{ffi::c_int, ptr::null_mut, sync::atomic::AtomicPtr};

use ulua_vm::{
  functions::{lua_callbacks::lua_callbacks, lua_rawcheckstack::lua_rawcheckstack},
  macros::lua_l_error::luaL_error,
  type_aliases::lua_state::lua_State,
};

// `replState` from Repl.cpp: the REPL's lua_State, used by the OS signal handler
// to arm the interrupt callback. Stored atomically so the async-signal handler
// (sigintHandler) can read it safely.
pub static REPL_STATE: AtomicPtr<lua_State> = AtomicPtr::new(null_mut());

/// # Safety
///
/// `l` must be a valid, active pointer to a `lua_State`.
// Ctrl-C handling. Matches the `interrupt` callback ABI on LuaCallbacks.
pub unsafe extern "C-unwind" fn sigint_callback(l: *mut lua_State, gc: c_int) {
  unsafe {
    if gc >= 0 {
      return;
    }

    (*lua_callbacks(l)).interrupt = None;

    lua_rawcheckstack(l, 1); // reserve space for error string
    luaL_error!(l, "Execution interrupted");
  }
}
