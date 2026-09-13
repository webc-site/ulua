use core::sync::atomic::Ordering;

use ulua_vm::{
  functions::{
    lua_callbacks::lua_callbacks, lua_pushcclosurek::lua_pushcclosurek,
    lua_singlestep::lua_singlestep,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
};

use crate::common::{
  functions::{
    conformance_debugger_breakpoint::conformance_debugger_breakpoint,
    conformance_debugger_debug_break::conformance_debugger_debug_break,
    conformance_debugger_debug_interrupt::conformance_debugger_debug_interrupt,
    conformance_debugger_debug_step::conformance_debugger_debug_step,
  },
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_setup(l: *mut lua_State) {
  unsafe {
    let cb = lua_callbacks(l);

    lua_singlestep(
      l,
      if CONFORMANCE_DEBUGGER_STATE.singlestep.load(Ordering::SeqCst) {
        1
      } else {
        0
      },
    );

    (*cb).debugstep = Some(conformance_debugger_debug_step);
    (*cb).debugbreak = Some(conformance_debugger_debug_break);
    (*cb).debuginterrupt = Some(conformance_debugger_debug_interrupt);

    lua_pushcclosurek(
      l,
      Some(conformance_debugger_breakpoint),
      c"breakpoint".as_ptr(),
      0,
      None,
    );
    lua_setglobal(l, c"breakpoint".as_ptr());
  }
}
