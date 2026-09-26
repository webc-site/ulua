use core::ptr::null;

use crate::{
  functions::{iter_aux::iter_aux, lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue},
  luaL_checkstring,
  macros::lua_pushcfunction::LUA_PUSHCFUNCTION,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkstring(l,1)` 要求索引 1 为字符串否则抛错回退；随后
/// `LUA_PUSHCFUNCTION`(C 闭包)/`lua_pushvalue(l,1)`/`lua_pushinteger` 连压 3 个值，`(*l).top` 后须留 ≥3 空槽
/// （由 utf8 库调用点保证）；push 可触发 GC。
/// cpp VM/src/lutf8lib.cpp:267
pub unsafe extern "C-unwind" fn iter_codes(l: *mut LuaState) -> i32 {
  unsafe {
    luaL_checkstring!(l, 1);
    LUA_PUSHCFUNCTION(l, Some(iter_aux), null());
    lua_pushvalue(l, 1);
    lua_pushinteger(l, 0);
    3
  }
}
