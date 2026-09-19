use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{
    lua_checkstack::lua_checkstack, lua_costatus::lua_costatus, lua_l_error_l::lua_l_error_l,
    lua_pushfstring_l::lua_pushfstring_l, lua_resume::lua_resume, lua_xmove::lua_xmove,
  },
  macros::{
    cast_int::cast_int, co_status_break::CO_STATUS_BREAK, co_status_error::CO_STATUS_ERROR,
    lua_minstack::LUA_MINSTACK, luai_maxcstack::LUAI_MAXCSTACK,
  },
  records::lua_state::lua_State,
};

const STATNAMES: [&str; 5] = ["running", "suspended", "normal", "dead", "dead"];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn auxresume(l: *mut lua_State, co: *mut lua_State, narg: i32) -> i32 {
  unsafe {
    // error handling for edge cases
    if (*co).status != LuaStatus::Yield as u8 {
      let status = lua_costatus(l, co);
      if status != LuaCoStatus::CoSus as i32 {
        lua_pushfstring_l(
          l,
          c"cannot resume %s coroutine".as_ptr(),
          format_args!("cannot resume {} coroutine", STATNAMES[status as usize]),
        );
        return CO_STATUS_ERROR;
      }
    }

    if narg != 0 {
      if lua_checkstack(co, narg) == 0 {
        lua_l_error_l(
          l,
          c"too many arguments to resume".as_ptr(),
          format_args!("too many arguments to resume"),
        );
      }
      lua_xmove(l, co, narg);
    } else {
      // coroutine might be completely full already
      if ((*co).top.offset_from((*co).base) as i32) > LUAI_MAXCSTACK {
        lua_l_error_l(
          l,
          c"too many arguments to resume".as_ptr(),
          format_args!("too many arguments to resume"),
        );
      }
    }

    (*co).singlestep = (*l).singlestep;

    let status = lua_resume(co, l, narg);
    if status == 0 || status == LuaStatus::Yield as i32 {
      let nres = cast_int!((*co).top.offset_from((*co).base));
      if nres != 0 {
        // +1 accounts for true/false status in resumefinish
        if nres + 1 > LUA_MINSTACK && lua_checkstack(l, nres + 1) == 0 {
          lua_l_error_l(
            l,
            c"too many results to resume".as_ptr(),
            format_args!("too many results to resume"),
          );
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
