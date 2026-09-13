use core::ptr::null_mut;

use ulua_vm::{
  functions::{
    lua_l_findtable::lua_l_findtable, lua_pushnil::lua_pushnil, lua_setfield::lua_setfield,
    lua_tolstring::lua_tolstring,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REQUIRED_CACHE_TABLE_KEY;

pub(crate) unsafe fn clear_cache_entry(l: *mut lua_State) -> i32 {
  unsafe {
    let cache_key = lua_tolstring(l, 1, null_mut());

    lua_l_findtable(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr(), 1);

    lua_pushnil(l);

    lua_setfield(l, -2, cache_key);

    lua_pop(l, 1);
  }
  0
}
