use ulua_vm::{
  functions::{
    lua_l_findtable::lua_l_findtable, lua_pushnil::lua_pushnil, lua_setfield::lua_setfield,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::lua_State,
};

use crate::functions::{c_str_prefix::with_c_str, cache_table_keys::REQUIRED_CACHE_TABLE_KEY};

/// # Safety
/// `l` 必须指向存活的 `lua_State`，栈顶参数布局由 C 调用约定保证。
pub(crate) unsafe fn clear_cache_entry(l: *mut lua_State) -> i32 {
  unsafe {
    // cpp 使用 luaL_checkstring：非字符串参数直接报错，
    // 而不是把空指针传给 lua_setfield（未定义行为）
    let cache_key = luaL_checkstring!(l, 1);

    with_c_str(REQUIRED_CACHE_TABLE_KEY, |table_key| {
      lua_l_findtable(l, LUA_REGISTRYINDEX, table_key, 1);
    });

    lua_pushnil(l);

    lua_setfield(l, -2, cache_key);

    lua_pop(l, 1);
  }
  0
}
