use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::callyieldable;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_arg_reuse(l: *mut LuaState) -> i32 {
  callyieldable(l, 2, 1)
}
