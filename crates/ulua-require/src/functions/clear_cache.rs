use ulua_vm::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_newtable::lua_newtable, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::{c_str_prefix::with_c_str, cache_table_keys::REQUIRED_CACHE_TABLE_KEY};

/// # Safety
/// `l` 必须指向存活的 `lua_State`。
pub(crate) unsafe fn clear_cache(l: *mut lua_State) -> i32 {
  unsafe {
    lua_newtable(l);

    with_c_str(REQUIRED_CACHE_TABLE_KEY, |table_key| {
      lua_setfield(l, LUA_REGISTRYINDEX, table_key);
    });
  }
  0
}
