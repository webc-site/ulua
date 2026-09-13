use ulua_vm::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_newtable::lua_newtable, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REQUIRED_CACHE_TABLE_KEY;

pub(crate) unsafe fn clear_cache(l: *mut lua_State) -> i32 {
  unsafe {
    lua_newtable(l);

    lua_setfield(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr());
  }
  0
}
