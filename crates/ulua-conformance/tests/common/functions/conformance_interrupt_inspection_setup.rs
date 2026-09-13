use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::lua_State};

use crate::common::functions::conformance_interrupt_inspection_interrupt::conformance_interrupt_inspection_interrupt;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_setup(l: *mut lua_State) {
  unsafe {
    (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_inspection_interrupt);
  }
}
