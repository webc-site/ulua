//! Source: `tests/FeedbackVector.test.cpp`

use core::ptr::null_mut;

use ulua_vm::records::{Proto::Proto, closure::Closure, lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn sealing_inliner(
  _l: *mut lua_State,
  _caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  null_mut()
}
