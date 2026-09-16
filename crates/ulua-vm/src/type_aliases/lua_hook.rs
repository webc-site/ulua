use crate::{records::lua_debug::LuaDebug, type_aliases::lua_state::lua_State};
pub type LuaHook = Option<unsafe extern "C-unwind" fn(l: *mut lua_State, ar: *mut LuaDebug)>;
