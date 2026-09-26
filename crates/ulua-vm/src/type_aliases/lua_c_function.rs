use crate::records::lua_state::LuaState;
pub type LuaCFunction = Option<unsafe extern "C-unwind" fn(l: *mut LuaState) -> i32>;
