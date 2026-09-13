use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_break::lua_break, lua_isyieldable::lua_isyieldable},
  records::lua_state::lua_State,
};

static mut SKIP_BREAK: bool = false;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_interrupt(
  l: *mut lua_State,
  gc: c_int,
) {
  unsafe {
    if gc >= 0 {
      return;
    }

    if lua_isyieldable(l) == 0 {
      return;
    }

    if !SKIP_BREAK {
      lua_break(l);
    }

    SKIP_BREAK = !SKIP_BREAK;
  }
}
