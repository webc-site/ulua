use ulua_vm::records::lua_state::LuaState;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_yield(_l: *mut LuaState) -> bool {
  true
}
