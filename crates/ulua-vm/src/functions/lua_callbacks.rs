use crate::records::{lua_callbacks::LuaCallbacks, lua_state::LuaState};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global` 指向其存活 `global_State`；返回的 `*mut LuaCallbacks` 指向
/// `(*g).cb`，生命周期随该 `global_State`（state 关闭前有效），调用方不得在 `l`/`g` 释放后使用返回值。纯取址，不分配、不抛错。
/// cpp VM/src/lapi.cpp:2163
pub unsafe fn lua_callbacks(l: *mut LuaState) -> *mut LuaCallbacks {
  unsafe { &mut (*(*l).global).cb as *mut LuaCallbacks }
}
