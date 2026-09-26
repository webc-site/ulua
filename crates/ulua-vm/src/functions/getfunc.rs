//! Source: `VM/src/lbaselib.cpp:108`
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
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const WHAT_F: &[u8] = b"f\0";

/// # Safety
///
/// `l` 必须指向存活 `LuaState` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn getfunc(l: *mut LuaState, opt: i32) {
  // Safety: 契约保证 `L` 存活且 lua_getinfo 语义的 level 帧定位有效（越界分支返回 nil），取回的函数 TValue 可读
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
      if lua_getinfo(l, level, WHAT_F.as_ptr().cast(), &mut ar) == 0 {
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
