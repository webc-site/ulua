//! Source: `VM/src/lcorolib.cpp:219`
//!
//! `coroutine.close` — close a dead/suspended thread: error if it is running or
//! normal, otherwise push `true` (and reset) for a clean thread, or `false` plus
//! the error object for an errored one, then reset it.

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{
    lua_costatus::lua_costatus, lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean,
    lua_pushstring::lua_pushstring, lua_resetthread::lua_resetthread, lua_tothread::lua_tothread,
    lua_xmove::lua_xmove,
  },
  macros::{
    lua_errerrmsg::LUA_ERRERRMSG, lua_l_argexpected::luaL_argexpected, lua_l_error::luaL_error,
    lua_lib_fn::lua_lib_fn, lua_memerrmsg::LUA_MEMERRMSG,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
pub(crate) unsafe fn coclose(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `L1` 为可关闭的非 main 协程存活状态，close 错误路径经 `L`（或 L1）报错、块内不改 main 线程栈
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, co.is_some(), 1, "thread");
    let co = co.expect("luaL_argexpected 已证 co 非空");

    let status = lua_costatus(l, co);
    if status != LuaCoStatus::CoFin as i32
      && status != LuaCoStatus::CoErr as i32
      && status != LuaCoStatus::CoSus as i32
    {
      let sname = LuaCoStatus::from_c_int(status).map_or("dead", LuaCoStatus::as_str);
      luaL_error!(l, "cannot close {} coroutine", sname);
    }

    if (*co).status as i32 == LuaStatus::Ok as i32 || (*co).status as i32 == LuaStatus::Yield as i32
    {
      lua_pushboolean(l, 1);
      lua_resetthread(co);
      1
    } else {
      lua_pushboolean(l, 0);

      if (*co).status as i32 == LuaStatus::ErrMem as i32 {
        lua_pushstring(l, LUA_MEMERRMSG.as_ptr().cast());
      } else if (*co).status as i32 == LuaStatus::ErrErr as i32 {
        lua_pushstring(l, LUA_ERRERRMSG.as_ptr().cast());
      } else if lua_gettop(co) != 0 {
        lua_xmove(co, l, 1); // move error message
      }

      lua_resetthread(co);
      2
    }
  }
}

lua_lib_fn!(pub(crate) fn coclose, coclose_arm);
