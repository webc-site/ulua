use core::ffi::c_char;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettable::lua_gettable, lua_l_findtable::lua_l_findtable, lua_pushlstring::lua_pushlstring,
    lua_remove::lua_remove, lua_type::lua_type,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REGISTERED_CACHE_TABLE_KEY;

pub(crate) unsafe fn check_registered_modules(l: *mut lua_State, path: &str) -> i32 {
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, REGISTERED_CACHE_TABLE_KEY.as_ptr(), 1);

    if path.bytes().all(|b| !b.is_ascii_uppercase()) {
      lua_pushlstring(l, path.as_ptr() as *const c_char, path.len());
    } else {
      let mut path_lower = path.as_bytes().to_vec();
      path_lower.make_ascii_lowercase();
      lua_pushlstring(l, path_lower.as_ptr() as *const c_char, path_lower.len());
    }

    lua_gettable(l, -2);

    if lua_type(l, -1) == LuaType::Nil as i32 {
      lua_pop(l, 2);
      0
    } else {
      lua_remove(l, -2);
      1
    }
  }
}
