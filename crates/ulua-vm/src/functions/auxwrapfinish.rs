//! Node: `cxx:Function:Luau.VM:VM/src/lcorolib.cpp:144:auxwrapfinish`
//! Source: `VM/src/lcorolib.cpp:144-157` (hand-ported)

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_insert::lua_insert,
    lua_isstring::lua_isstring, lua_l_where::lua_l_where,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn auxwrapfinish(l: *mut lua_State, r: i32) -> i32 {
  unsafe {
    if r < 0 {
      if lua_isstring(l, -1) != 0 {
        lua_l_where(l, 1);
        lua_insert(l, -2);
        lua_concat(l, 2);
      }
      lua_error(l);
    }
    r
  }
}
