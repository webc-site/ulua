use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState`，仅读其 `(*l).n_ccalls` 与 `(*l).base_ccalls` 计数字段判断可否 yield。
/// cpp `ldo.cpp:869`。
pub unsafe fn lua_isyieldable(l: *mut LuaState) -> i32 {
  unsafe {
    if (*l).n_ccalls <= (*l).base_ccalls {
      1
    } else {
      0
    }
  }
}
