//! Node: `cxx:Function:Luau.VM:VM/src/lcorolib.cpp:219:coclose`
//!
//! `coroutine.close` — close a dead/suspended thread: error if it is running or
//! normal, otherwise push `true` (and reset) for a clean thread, or `false` plus
//! the error object for an errored one, then reset it.

use core::ffi::c_int;

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{
    lua_costatus::lua_costatus, lua_gettop::lua_gettop, lua_pushboolean::lua_pushboolean,
    lua_pushstring::lua_pushstring, lua_resetthread::lua_resetthread, lua_tothread::lua_tothread,
    lua_xmove::lua_xmove,
  },
  macros::{lua_l_argexpected::luaL_argexpected, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub(crate) unsafe extern "C-unwind" fn coclose(l: *mut lua_State) -> c_int {
  unsafe {
    let co = lua_tothread(l, 1);
    luaL_argexpected!(l, !co.is_null(), 1, "thread");

    let status = lua_costatus(l, co);
    if status != LuaCoStatus::CoFin as c_int
      && status != LuaCoStatus::CoErr as c_int
      && status != LuaCoStatus::CoSus as c_int
    {
      let sname = match status {
        0 => "running",
        1 => "suspended",
        2 => "normal",
        _ => "dead",
      };
      luaL_error!(l, "cannot close {} coroutine", sname);
    }

    if (*co).status as c_int == LuaStatus::Ok as c_int
      || (*co).status as c_int == LuaStatus::Yield as c_int
    {
      lua_pushboolean(l, 1);
      lua_resetthread(co);
      1
    } else {
      lua_pushboolean(l, 0);

      if (*co).status as c_int == LuaStatus::ErrMem as c_int {
        lua_pushstring(l, c"not enough memory".as_ptr());
      } else if (*co).status as c_int == LuaStatus::ErrErr as c_int {
        lua_pushstring(l, c"error in error handling".as_ptr());
      } else if lua_gettop(co) != 0 {
        lua_xmove(co, l, 1); // move error message
      }

      lua_resetthread(co);
      2
    }
  }
}
