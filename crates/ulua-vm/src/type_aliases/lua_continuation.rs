use core::ffi::c_int;

use crate::type_aliases::lua_state::lua_State;
pub type LuaContinuation =
  Option<unsafe extern "C-unwind" fn(l: *mut lua_State, status: c_int) -> c_int>;
