use core::ffi::c_void;

use ulua_vm::records::lua_state::LuaState;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn userdata_alignment_dtor(_l: *mut LuaState, _data: *mut c_void) {}
