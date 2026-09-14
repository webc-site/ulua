//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:73:pseudo2addr`
//! Source: `VM/src/lapi.cpp:65-97` (hand-ported; includes the file-static
//! `getcurrenv` helper inlined here since it was never a graph node)

use core::ffi::c_int;

use crate::{
  macros::{
    api_check::api_check, curr_func::curr_func, lua_environindex::LUA_ENVIRONINDEX,
    lua_globalsindex::LUA_GLOBALSINDEX, lua_ispseudo::lua_ispseudo,
    lua_o_nilobject::luaO_nilobject, lua_registryindex::LUA_REGISTRYINDEX, registry::registry,
    sethvalue::sethvalue,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

unsafe fn getcurrenv(l: *mut lua_State) -> *mut LuaTable {
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

pub(crate) unsafe fn pseudo_2_addr(l: *mut lua_State, idx: c_int) -> StkId {
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
          luaO_nilobject as *mut TValue
        }
      }
    }
  }
}
