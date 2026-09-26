use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`，`(*l).top` 与 `(*l).base` 须为同一栈数组内的有效指针且 `top>=base`
/// （仅做 `offset_from` 差值，不解引用元素）。cpp `lapi.cpp:262`。
pub unsafe fn lua_gettop(l: *mut LuaState) -> i32 {
  unsafe { (*l).top.offset_from((*l).base) as i32 }
}
