use crate::{
  functions::{lua_h_get::lua_h_get, lua_h_newkey::lua_h_newkey},
  macros::{
    cast_to::cast_to, invalidate_t_mcache::invalidate_tmcache, lua_o_nilobject::luaO_nilobject,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
///
/// `l` and `t` must be valid pointers.
pub unsafe fn lua_h_set(l: *mut lua_State, t: *mut LuaTable, key: *const TValue) -> *mut TValue {
  unsafe {
    let p = lua_h_get(t, key);
    invalidate_tmcache(t);

    if p != luaO_nilobject {
      cast_to!(*mut TValue, p)
    } else {
      lua_h_newkey(l, t, key)
    }
  }
}

pub use lua_h_set as luaH_set;
