use crate::{
  functions::{lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushnumber::lua_pushnumber},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `lua_State` 且栈 index 1 为可转 64 位整数的值（`luaL_checkinteger64` 读槽并可抛错），
/// `lua_pushnumber` 可能扩栈，须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:52`。
pub unsafe fn int64_tonumber(l: *mut LuaState) -> i32 {
  unsafe {
    let x = lua_l_checkinteger_64(l, 1);
    lua_pushnumber(l, x as f64);
    1
  }
}

lua_lib_fn!(pub fn int64_tonumber, int64_tonumber_arm);
