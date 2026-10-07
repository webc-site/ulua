use std::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_new_userdata_overflow_dtor(
  _l: *mut LuaState,
  _data: *mut c_void,
) {
}
