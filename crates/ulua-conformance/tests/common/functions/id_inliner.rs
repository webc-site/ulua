use ulua_vm::records::{Closure::Closure, Proto::Proto, lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn id_inliner(
  _l: *mut lua_State,
  caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  unsafe { (*caller).inner.l.p }
}
