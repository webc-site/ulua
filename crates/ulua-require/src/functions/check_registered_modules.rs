use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettable::lua_gettable, lua_l_findtable::lua_l_findtable, lua_remove::lua_remove,
    lua_type::lua_type,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::{
  c_str_prefix::{push_lowered_c_str, with_c_str},
  cache_table_keys::REGISTERED_CACHE_TABLE_KEY,
};

/// # Safety
/// `l` 必须指向存活的 `lua_State`；函数在 Lua 栈上压入/弹出临时值。
/// `path` 为 require 路径原始字节（cpp `checkRegisteredModules` 收 `const char*`）。
/// 命中缓存时返回 true，且命中值留在栈顶（cpp 返回 1 时同样留在栈顶）。
pub(crate) unsafe fn check_registered_modules(l: *mut lua_State, path: &[u8]) -> bool {
  unsafe {
    with_c_str(REGISTERED_CACHE_TABLE_KEY, |table_key| {
      lua_l_findtable(l, LUA_REGISTRYINDEX, table_key, 1);
    });

    // 无大写时零分配直推；有则转小写后推入（cpp 同为 ASCII 字节小写归一查缓存）
    push_lowered_c_str(l, path);

    lua_gettable(l, -2);

    if lua_type(l, -1) == LuaType::Nil as i32 {
      lua_pop(l, 2);
      false
    } else {
      lua_remove(l, -2);
      true
    }
  }
}
