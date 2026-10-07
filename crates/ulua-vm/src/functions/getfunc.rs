//! Source: `VM/src/lbaselib.cpp:108`
//!
//! `getfunc` — resolve the function argument for `getfenv`/`setfenv`: either the
//! explicit function at slot 1, or the function at stack `level` (via
//! `lua_getinfo`'s `f` option, which pushes it). Used by the env builtins.

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 C 字符串类型）。
use core::mem::zeroed;

use crate::{
  functions::{lua_getinfo::lua_getinfo, lua_l_optinteger::lua_l_optinteger},
  macros::lua_l_error::luaL_error,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};
const WHAT_F: &[u8] = b"f\0";

/// # Safety
///
/// `l` 必须指向存活 `LuaState` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn getfunc(l: *mut LuaState, opt: i32) {
  // SAFETY: 契约保证 `L` 存活且 lua_getinfo 语义的 level 帧定位有效（越界分支返回 nil），取回的函数 TValue 可读
  unsafe {
    if (*l).is_function(1) {
      (*l).push_value(1);
    } else {
      let mut ar: LuaDebug = zeroed();
      let level: i32 = if opt != 0 {
        lua_l_optinteger(&mut *l, 1, 1)
      } else {
        (*l).check_integer(1)
      };
      (*l).arg_check(level >= 0, 1, "level must be non-negative");
      if lua_getinfo(l, level, WHAT_F.as_ptr().cast(), &mut ar) == 0 {
        (*l).arg_error(1, "invalid level");
      }
      if (*l).is_nil(-1) {
        luaL_error!(
          &mut *l,
          "no function environment for tail call at level {}",
          level
        );
      }
    }
  }
}
