use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`，仅把 `enabled!=0` 写入 `(*l).singlestep` 布尔字段，不触碰其它内存。
/// cpp `ldebug.cpp:482`。
pub unsafe fn lua_singlestep(l: *mut LuaState, enabled: i32) {
  unsafe {
    (*l).singlestep = enabled != 0;
  }
}
