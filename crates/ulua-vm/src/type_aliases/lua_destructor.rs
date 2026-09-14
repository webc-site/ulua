use core::ffi::c_void;

use crate::type_aliases::lua_state::lua_State;
pub type LuaDestructor =
  Option<unsafe extern "C-unwind" fn(l: *mut lua_State, userdata: *mut c_void)>;
