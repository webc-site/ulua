use core::ffi::c_int;

use ulua_vm::records::lua_state::lua_State;
pub type Coverage = unsafe extern "C-unwind" fn(*mut lua_State, c_int);
