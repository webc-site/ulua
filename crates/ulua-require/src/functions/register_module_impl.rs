use core::{ffi::c_char, slice::from_raw_parts};

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_insert::lua_insert, lua_l_argerror_l::lua_l_argerror_l,
    lua_l_checklstring::lua_l_checklstring, lua_l_findtable::lua_l_findtable,
    lua_pushlstring::lua_pushlstring, lua_replace::lua_replace, lua_settable::lua_settable,
  },
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REGISTERED_CACHE_TABLE_KEY;

/// # Safety
/// `l` 必须指向存活的 `lua_State`，栈上需有注册模块所需的两个参数。
pub(crate) unsafe fn register_module_impl(l: *mut lua_State) -> i32 {
  unsafe {
    if lua_gettop(l) != 2 {
      luaL_error!(
        l,
        "expected 2 arguments: aliased require path and desired result"
      );
    }

    let mut len = 0usize;
    let path = lua_l_checklstring(l, 1, &mut len);
    if len == 0 || *path != b'@' as c_char {
      lua_l_argerror_l(l, 1, "path must begin with '@'");
    }

    // cpp 按 std::string 字节小写归一，slice::to_ascii_lowercase 等价
    let path_lower = from_raw_parts(path as *const u8, len).to_ascii_lowercase();

    lua_pushlstring(l, path_lower.as_ptr() as *const c_char, path_lower.len());
    lua_replace(l, 1);

    lua_l_findtable(l, LUA_REGISTRYINDEX, REGISTERED_CACHE_TABLE_KEY.as_ptr(), 1);
    lua_insert(l, 1);
    lua_settable(l, 1);
    lua_pop(l, 1);

    0
  }
}
