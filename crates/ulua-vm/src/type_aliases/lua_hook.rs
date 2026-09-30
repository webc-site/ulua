use crate::records::{lua_debug::LuaDebug, lua_state::LuaState};
pub type LuaHook = Option<unsafe extern "C-unwind" fn(l: *mut LuaState, ar: *mut LuaDebug)>;
