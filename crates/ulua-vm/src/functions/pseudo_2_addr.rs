//! Source: `VM/src/lapi.cpp:65-97` (hand-ported; includes the file-static
//! `getcurrenv` helper inlined here since it was never a graph node)

use crate::{
  macros::{
    api_check::api_check, curr_func::curr_func, lua_environindex::LUA_ENVIRONINDEX,
    lua_globalsindex::LUA_GLOBALSINDEX, lua_ispseudo::lua_ispseudo,
    lua_o_nilobject::LUA_O_NILOBJECT, lua_registryindex::LUA_REGISTRYINDEX, registry::registry,
    sethvalue::sethvalue,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState`，`(*l).gt` 有效；当 `(*l).ci != (*l).base_ci` 时当前调用帧
/// `func` 须为闭包（`curr_func` 取 `(*cl).env`），否则回退到 `gt`。cpp `lapi.cpp:81`。
unsafe fn getcurrenv(l: *mut LuaState) -> *mut LuaTable {
  unsafe {
    if (*l).ci == (*l).base_ci {
      // no enclosing function? use global table as environment
      (*l).gt
    } else {
      let func = curr_func!(l);
      (*func).env
    }
  }
}

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global.pseudotemp`、`registry`、`gt` 均有效；`idx`
/// 须满足 `lua_ispseudo(idx)`（`api_check` 保证），LUA_ENVIRONINDEX/GLOBALSINDEX 分支会经
/// `sethvalue` 写 pseudotemp 并可能触发写屏障；越界伪索引返回 `luaO_nilobject`。cpp `lapi.cpp:89`。
pub(crate) unsafe fn pseudo_2_addr(l: *mut LuaState, idx: i32) -> StkId {
  unsafe {
    api_check!(l, lua_ispseudo(idx));
    match idx {
      // pseudo-indices
      LUA_REGISTRYINDEX => registry!(l) as *const TValue as *mut TValue,
      LUA_ENVIRONINDEX => {
        let tmp = &mut (*(*l).global).pseudotemp as *mut TValue;
        sethvalue!(l, tmp, getcurrenv(l));
        tmp
      }
      LUA_GLOBALSINDEX => {
        let tmp = &mut (*(*l).global).pseudotemp as *mut TValue;
        sethvalue!(l, tmp, (*l).gt);
        tmp
      }
      _ => {
        let func = curr_func!(l);
        let idx = LUA_GLOBALSINDEX - idx;
        if idx <= (*func).nupvalues as i32 {
          let c = &mut (*func).inner.c;
          c.upvals.as_mut_ptr().add((idx - 1) as usize)
        } else {
          LUA_O_NILOBJECT as *mut TValue
        }
      }
    }
  }
}
