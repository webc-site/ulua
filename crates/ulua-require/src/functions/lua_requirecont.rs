use ulua_vm::{
  functions::{
    lua_getfield::lua_getfield, lua_gettop::lua_gettop, lua_pushvalue::lua_pushvalue,
    lua_setfield::lua_setfield,
  },
  macros::{
    lua_l_checkstring::luaL_checkstring, lua_l_error::luaL_error, lua_pop::lua_pop,
    lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::lua_State,
};

use crate::functions::cache_table_keys::REQUIRED_CACHE_TABLE_KEY;

pub const K_REQUIRE_STACK_VALUES: i32 = 4;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe extern "C-unwind" fn lua_requirecont(l: *mut lua_State, _status: i32) -> i32 {
  unsafe {
    ulua_common::LUAU_ASSERT!(lua_gettop(l) >= K_REQUIRE_STACK_VALUES);
    let num_results = lua_gettop(l) - K_REQUIRE_STACK_VALUES;
    let cache_key = luaL_checkstring!(l, 2);

    if num_results > 1 {
      luaL_error!(l, "module must return a single value");
      return 0;
    }

    if num_results == 1 {
      lua_getfield(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY.as_ptr());
      lua_pushvalue(l, -2);
      lua_setfield(l, -2, cache_key);
      lua_pop(l, 1);
    }

    num_results
  }
}
