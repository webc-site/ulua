use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 `lua_State` 且栈 index 1 为可转 64 位整数的值（`luaL_checkinteger64` 读槽并可抛错），
/// `lua_pushnumber` 可能扩栈，须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:52`。
pub unsafe fn int64_tonumber(l: *mut LuaState) -> i32 {
  unsafe {
    let x = (*l).check_integer_64(1);
    (*l).push_number(x as f64);
    1
  }
}

lua_lib_fn!(pub fn int64_tonumber, int64_tonumber_arm);
