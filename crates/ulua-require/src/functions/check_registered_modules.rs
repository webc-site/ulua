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

/// # Safety
/// `l` 必须指向存活的 `lua_State`；函数在 Lua 栈上压入/弹出临时值。
pub(crate) unsafe fn check_registered_modules(l: *mut lua_State, path: &str) -> i32 {
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, REGISTERED_CACHE_TABLE_KEY.as_ptr(), 1);

    // 无大写时零分配直推；有则转小写后推入（cpp 同为小写归一查缓存）
    if path.bytes().any(|b| b.is_ascii_uppercase()) {
      let path_lower = path.to_ascii_lowercase();
      lua_pushlstring(l, path_lower.as_ptr() as *const c_char, path_lower.len());
    } else {
      lua_pushlstring(l, path.as_ptr() as *const c_char, path.len());
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
