use core::ffi::c_void;

use crate::type_aliases::lua_state::LuaState;
pub fn dealloc_type_user_data(_l: *mut LuaState, _data: *mut c_void) {
  // only non-owning pointers into an arena is stored
}
