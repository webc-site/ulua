use crate::records::lua_state::LuaState;
pub type LuaContinuation =
  Option<unsafe extern "C-unwind" fn(l: *mut LuaState, status: i32) -> i32>;
