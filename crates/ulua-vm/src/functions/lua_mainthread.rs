use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState` 且其 `(*l).global` 有效（仅读 `(*(*l).global).mainthread`，不解引用元素）。
/// cpp `lapi.cpp:247`。
pub unsafe fn lua_mainthread(l: *mut LuaState) -> *mut LuaState {
  unsafe { (*(*l).global).mainthread }
}
