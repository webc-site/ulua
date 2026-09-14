use alloc::ffi::CString;

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::{c_str_prefix::c_str_prefix, cache_table_keys::REQUIRED_CACHE_TABLE_KEY};

/// # Safety
/// `l` 必须指向存活的 `lua_State`；函数在 Lua 栈上压入/弹出临时值。
pub(crate) unsafe fn is_cached(l: *mut lua_State, key: &str) -> bool {
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr(), 1);

    // 与 cpp `key.c_str()` 一致：按首个 NUL 截断，截断后必无 NUL
    let key_c = CString::new(c_str_prefix(key)).unwrap();
    lua_getfield(l, -1, key_c.as_ptr());

    let cached = lua_type(l, -1) != (LuaType::Nil as i32);

    lua_pop(l, 2);

    cached
  }
}
