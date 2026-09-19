use ulua_vm::records::lua_state::lua_State;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_yield(_l: *mut lua_State) -> bool {
  true
}
