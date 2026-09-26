use core::ptr::null;

use crate::{
  functions::{
    gmatch_aux::gmatch_aux_arm, lua_pushinteger::lua_pushinteger, lua_settop::lua_settop,
  },
  luaL_checkstring,
  macros::{lua_lib_fn::lua_lib_fn, lua_pushcclosure::lua_pushcclosure},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn gmatch(l: *mut LuaState) -> i32 {
  unsafe {
    luaL_checkstring!(l, 1);
    luaL_checkstring!(l, 2);
    lua_settop(l, 2);
    lua_pushinteger(l, 0);
    lua_pushcclosure(l, Some(gmatch_aux_arm), null(), 3);
    1
  }
}

lua_lib_fn!(pub fn gmatch, gmatch_arm);
