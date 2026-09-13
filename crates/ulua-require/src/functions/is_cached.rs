use alloc::ffi::CString;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REQUIRED_CACHE_TABLE_KEY;

pub(crate) unsafe fn is_cached(l: *mut lua_State, key: &str) -> bool {
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr(), 1);

    let key_c = match CString::new(key) {
      Ok(c) => c,
      Err(_) => {
        lua_pop(l, 1);
        return false;
      }
    };
    lua_getfield(l, -1, key_c.as_ptr());

    let cached = lua_type(l, -1) != (LuaType::Nil as i32);

    lua_pop(l, 2);

    cached
  }
}
