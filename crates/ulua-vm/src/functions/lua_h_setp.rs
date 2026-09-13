use core::{ffi::c_void, mem::zeroed};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_h_getp::lua_h_getp, newkey::newkey},
  macros::{cast_to::cast_to, lua_o_nilobject::luaO_nilobject},
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, t_value::TValue},
};

pub(crate) unsafe fn lua_h_setp(
  l: *mut lua_State,
  t: *mut LuaTable,
  key: *mut c_void,
  tag: i32,
) -> *mut TValue {
  unsafe {
    let p = lua_h_getp(t, key, tag);

    if p != luaO_nilobject {
      cast_to!(*mut TValue, p)
    } else {
      let mut k: TValue = zeroed();

      k.value.p = key;
      k.extra[0] = tag;
      k.tt = LuaType::LightUserData as i32;

      newkey(l, t, &k)
    }
  }
}
