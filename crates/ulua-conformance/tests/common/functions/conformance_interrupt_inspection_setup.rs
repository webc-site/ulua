use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_interrupt_inspection_interrupt::conformance_interrupt_inspection_interrupt,
  safe_api::callbacks_mut,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_setup(l: *mut LuaState) {
  callbacks_mut(l).interrupt = Some(conformance_interrupt_inspection_interrupt);
}
