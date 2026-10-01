use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_tag_method_error_debug_protected_error::conformance_tag_method_error_debug_protected_error,
  safe_api::callbacks_mut,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_setup(l: *mut LuaState) {
  callbacks_mut(l).debugprotectederror = Some(conformance_tag_method_error_debug_protected_error);
}
