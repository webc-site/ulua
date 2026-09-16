//! Node: `cxx:Function:Luau.VM:VM/src/lbaselib.cpp:108:getfunc`
//!
//! `getfunc` — resolve the function argument for `getfenv`/`setfenv`: either the
//! explicit function at slot 1, or the function at stack `level` (via
//! `lua_getinfo`'s `f` option, which pushes it). Used by the env builtins.

use core::mem::zeroed;

use crate::{
  functions::{
    lua_getinfo::lua_getinfo, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_optinteger::lua_l_optinteger, lua_pushvalue::lua_pushvalue,
  },
  macros::{
    lua_isfunction::lua_isfunction, lua_isnil::lua_isnil, lua_l_argcheck::luaL_argcheck,
    lua_l_argerror::luaL_argerror, lua_l_error::luaL_error,
  },
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getfunc(l: *mut lua_State, opt: i32) {
  unsafe {
    if lua_isfunction!(l, 1) {
      lua_pushvalue(l, 1);
    } else {
      let mut ar: LuaDebug = zeroed();
      let level: i32 = if opt != 0 {
        lua_l_optinteger(l, 1, 1)
      } else {
        lua_l_checkinteger(l, 1)
      };
      luaL_argcheck!(l, level >= 0, 1, "level must be non-negative");
      if lua_getinfo(l, level, c"f".as_ptr(), &mut ar) == 0 {
        luaL_argerror!(l, 1, "invalid level");
      }
      if lua_isnil!(l, -1) {
        luaL_error!(
          l,
          "no function environment for tail call at level {}",
          level
        );
      }
    }
  }
}
