use core::sync::atomic::Ordering;

use ulua_vm::records::{lua_debug::LuaDebug, lua_state::lua_State};

use crate::common::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_debug_interrupt(
  _l: *mut lua_State,
  ar: *mut LuaDebug,
) {
  unsafe {
    assert!(
      CONFORMANCE_DEBUGGER_STATE
        .interruptedthread
        .load(Ordering::SeqCst)
        .is_null()
    );
    assert!(!(*ar).userdata.is_null());

    CONFORMANCE_DEBUGGER_STATE
      .interruptedthread
      .store((*ar).userdata as *mut lua_State, Ordering::SeqCst);
  }
}
