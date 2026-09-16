use core::sync::atomic::Ordering;

use ulua_vm::{
  functions::{lua_break::lua_break, lua_debugtrace::lua_debugtrace},
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

use crate::common::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_debug_break(
  l: *mut lua_State,
  _ar: *mut LuaDebug,
) {
  unsafe {
    let breakhits = CONFORMANCE_DEBUGGER_STATE
      .breakhits
      .fetch_add(1, Ordering::SeqCst)
      + 1;

    lua_debugtrace(l);

    if breakhits % 2 == 1 {
      lua_break(l);
    }
  }
}
