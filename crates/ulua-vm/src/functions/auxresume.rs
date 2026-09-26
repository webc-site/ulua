use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{
    lua_checkstack::lua_checkstack, lua_costatus::lua_costatus,
    lua_pushfstring_l::lua_pushfstring_l, lua_resume::lua_resume, lua_xmove::lua_xmove,
  },
  macros::{
    co_status_break::CO_STATUS_BREAK, co_status_error::CO_STATUS_ERROR, lua_l_error::luaL_error,
    lua_minstack::LUA_MINSTACK, luai_maxcstack::LUAI_MAXCSTACK,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l`、`co` 须均为存活 `LuaState`（`co` 为待恢复协程）且调用点处于受保护帧：本函数可读改二者 `(*.top)`/
/// `(*.base)`/`(*.status)`/`singlestep`，`lua_xmove(l,co,narg)` 要求 `l` 顶至少有 narg 个可移实参、`co` 已经
/// `lua_checkstack` 腾出 narg 槽；错误分支经 `lua_l_error_l`/`lua_pushfstring_l` 抛错或压栈需 `l` 栈余量。
/// `lua_resume` 可再入 GC/抛错。返回 CO_STATUS_* 码。
/// cpp VM/src/lcorolib.cpp:116
pub(crate) unsafe fn auxresume(l: *mut LuaState, co: *mut LuaState, narg: i32) -> i32 {
  unsafe {
    // error handling for edge cases
    if (*co).status != LuaStatus::Yield as u8 {
      let status = lua_costatus(l, co);
      if status != LuaCoStatus::CoSus as i32 {
        let sname = LuaCoStatus::from_c_int(status).map_or("dead", LuaCoStatus::as_str);
        lua_pushfstring_l(l, format_args!("cannot resume {} coroutine", sname));
        return CO_STATUS_ERROR;
      }
    }

    if narg != 0 {
      if lua_checkstack(co, narg) == 0 {
        luaL_error!(l, "too many arguments to resume");
      }
      lua_xmove(l, co, narg);
    } else {
      // coroutine might be completely full already
      if ((*co).top.offset_from((*co).base) as i32) > LUAI_MAXCSTACK {
        luaL_error!(l, "too many arguments to resume");
      }
    }

    (*co).singlestep = (*l).singlestep;

    let status = lua_resume(co, l, narg);
    if status == 0 || status == LuaStatus::Yield as i32 {
      let nres = (*co).top.offset_from((*co).base) as i32;
      if nres != 0 {
        // +1 accounts for true/false status in resumefinish
        if nres + 1 > LUA_MINSTACK && lua_checkstack(l, nres + 1) == 0 {
          luaL_error!(l, "too many results to resume");
        }
        lua_xmove(co, l, nres); // move yielded values
      }
      nres
    } else if status == LuaStatus::Break as i32 {
      CO_STATUS_BREAK
    } else {
      lua_xmove(co, l, 1); // move error message
      CO_STATUS_ERROR
    }
  }
}
