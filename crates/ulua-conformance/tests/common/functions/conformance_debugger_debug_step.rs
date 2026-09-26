use core::sync::atomic::Ordering;

use ulua_vm::records::{lua_debug::LuaDebug, lua_state::LuaState};

use crate::common::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_debug_step(
  _l: *mut LuaState,
  _ar: *mut LuaDebug,
) {
  CONFORMANCE_DEBUGGER_STATE
    .stephits
    .fetch_add(1, Ordering::SeqCst);
}
