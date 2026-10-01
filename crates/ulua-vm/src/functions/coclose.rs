//! Source: `VM/src/lcorolib.cpp:219`
//!
//! `coroutine.close` — close a dead/suspended thread: error if it is running or
//! normal, otherwise push `true` (and reset) for a clean thread, or `false` plus
//! the error object for an errored one, then reset it.

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{lua_costatus::lua_costatus, lua_resetthread::lua_resetthread, lua_xmove::lua_xmove},
  macros::{
    lua_errerrmsg::LUA_ERRERRMSG_STR, lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn,
    lua_memerrmsg::LUA_MEMERRMSG_STR,
  },
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
pub(crate) unsafe fn coclose(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `L1` 为可关闭的非 main 协程存活状态，close 错误路径经 `L`（或 L1）报错、块内不改 main 线程栈
  unsafe {
    // 非 thread 实参直接走 `type_error`（`-> !`）：let-else 让「取不到即报错」的
    // 控制流由类型承载，不再需要 unwrap 的二次论证。
    let Some(co) = (*l).to_thread(1) else {
      (*l).type_error(1, "thread")
    };

    let status = lua_costatus(&*l, &*co);
    if status != LuaCoStatus::CoFin as i32
      && status != LuaCoStatus::CoErr as i32
      && status != LuaCoStatus::CoSus as i32
    {
      let sname = LuaCoStatus::from_c_int(status).map_or("dead", LuaCoStatus::as_str);
      luaL_error!(l, "cannot close {} coroutine", sname);
    }

    if (*co).status as i32 == LuaStatus::Ok as i32 || (*co).status as i32 == LuaStatus::Yield as i32
    {
      (*l).push_boolean(true);
      lua_resetthread(co);
      1
    } else {
      (*l).push_boolean(false);

      if (*co).status as i32 == LuaStatus::ErrMem as i32 {
        (*l).push_str(LUA_MEMERRMSG_STR);
      } else if (*co).status as i32 == LuaStatus::ErrErr as i32 {
        (*l).push_str(LUA_ERRERRMSG_STR);
      } else if (*co).get_top() != 0 {
        lua_xmove(co, l, 1); // move error message
      }

      lua_resetthread(co);
      2
    }
  }
}

lua_lib_fn!(pub(crate) fn coclose, coclose_arm);
