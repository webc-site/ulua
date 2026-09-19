use ulua_vm::{functions::lua_callbacks::lua_callbacks, records::lua_state::lua_State};

use crate::common::functions::conformance_tag_method_error_debug_protected_error::conformance_tag_method_error_debug_protected_error;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_setup(l: *mut lua_State) {
  unsafe {
    (*lua_callbacks(l)).debugprotectederror =
      Some(conformance_tag_method_error_debug_protected_error);
  }
}
