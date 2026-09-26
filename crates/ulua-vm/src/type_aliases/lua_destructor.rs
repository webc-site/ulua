use core::ffi::c_void;

use crate::records::lua_state::LuaState;
pub type LuaDestructor =
  Option<unsafe extern "C-unwind" fn(l: *mut LuaState, userdata: *mut c_void)>;
