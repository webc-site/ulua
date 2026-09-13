use ulua_vm::{functions::lua_l_callyieldable::lua_l_callyieldable, records::lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_arg_reuse(l: *mut lua_State) -> i32 {
  unsafe { lua_l_callyieldable(l, 2, 1) }
}
