use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettable::lua_gettable, lua_l_findtable::lua_l_findtable, lua_pushlstring::lua_pushlstring,
    lua_type::lua_type,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::{
  c_str_prefix::{c_str_prefix, with_c_str},
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY,
};

/// # Safety
/// `l` 必须指向存活的 `lua_State`；函数在 Lua 栈上压入/弹出临时值。
pub(crate) unsafe fn is_cached(l: *mut lua_State, key: &[u8]) -> bool {
  unsafe {
    with_c_str(REQUIRED_CACHE_TABLE_KEY, |table_key| {
      lua_l_findtable(l, LUA_REGISTRYINDEX, table_key, 1);
    });

    // 与 cpp `lua_getfield(L, -1, key.c_str())` 一致：按首个 NUL 截断，
    // 经 pushlstring + gettable 零拷贝等价实现（免 NUL 补齐）
    let key = c_str_prefix(key);
    lua_pushlstring(l, key.as_ptr().cast(), key.len());
    lua_gettable(l, -2);

    let cached = lua_type(l, -1) != (LuaType::Nil as i32);

    lua_pop(l, 2);

    cached
  }
}
