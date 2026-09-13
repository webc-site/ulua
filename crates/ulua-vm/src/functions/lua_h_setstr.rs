use core::mem::zeroed;

use crate::{
  functions::{lua_h_getstr::lua_h_getstr, newkey::newkey},
  macros::{
    cast_to::cast_to, invalidate_t_mcache::invalidate_tmcache, lua_o_nilobject::luaO_nilobject,
    setsvalue::setsvalue,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, t_string::tstring, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_h_setstr(l: *mut lua_State, t: *mut LuaTable, key: *mut tstring) -> *mut TValue {
  unsafe {
    let p = lua_h_getstr(t, key);
    invalidate_tmcache(t);

    if p != luaO_nilobject {
      cast_to!(*mut TValue, p)
    } else {
      let mut k: TValue = zeroed();
      setsvalue!(l, &mut k, key);

      newkey(l, t, &k)
    }
  }
}
