use core::ptr::null;

use crate::{
  functions::{lua_g_runerror_l::lua_g_runerror_l, luai_vecisnan::luai_vecisnan, newkey::newkey},
  macros::{
    luai_numisnan::luai_numisnan, nvalue::nvalue, ttisnil::ttisnil, ttisnumber::ttisnumber,
    ttisvector::ttisvector, vvalue::vvalue,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, t_value::TValue},
};

pub(crate) unsafe fn lua_h_newkey(
  l: *mut lua_State,
  t: *mut LuaTable,
  key: *const TValue,
) -> *mut TValue {
  unsafe {
    if ttisnil!(key) {
      lua_g_runerror_l(l, null(), format_args!("table index is nil"));
    } else if ttisnumber!(key) && luai_numisnan(nvalue!(key)) {
      lua_g_runerror_l(l, null(), format_args!("table index is NaN"));
    } else if ttisvector!(key) && luai_vecisnan(vvalue!(key).as_ptr()) {
      lua_g_runerror_l(l, null(), format_args!("table index contains NaN"));
    }

    newkey(l, t, key)
  }
}
