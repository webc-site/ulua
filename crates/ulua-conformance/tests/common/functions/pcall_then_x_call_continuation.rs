use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_callyieldable::lua_callyieldable, lua_l_checkinteger::luaL_checkinteger,
    lua_l_checkstack::lua_l_checkstack, lua_pcallyieldable::lua_pcallyieldable,
    lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue, lua_replace::lua_replace,
  },
  macros::{
    lua_multret::LUA_MULTRET, lua_tointeger::lua_tointeger, lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn pcall_then_x_call_continuation(
  l: *mut lua_State,
  status: i32,
) -> i32 {
  unsafe {
    lua_l_checkstack(l, 1, "pcallThenCallContinuation");

    let pcall_variant = lua_tointeger!(l, lua_upvalueindex(1));
    let state = luaL_checkinteger(l, 3);

    if state == 0 {
      if status != LuaStatus::Ok as i32 {
        lua_pushinteger(l, -1);
        lua_replace(l, 4);
      } else {
        lua_replace(l, 4);
      }

      lua_pushinteger(l, 1);
      lua_replace(l, 3);

      lua_pushvalue(l, 2); // call second function
      if pcall_variant != 0 {
        lua_pcallyieldable(l, 0, LUA_MULTRET, 0)
      } else {
        lua_callyieldable(l, 0, LUA_MULTRET)
      }
    } else {
      let multiplier = luaL_checkinteger(l, 4);
      let value = if status != LuaStatus::Ok as i32 {
        LUAU_ASSERT!(pcall_variant != 0);
        -1
      } else {
        lua_tointeger!(l, -1)
      };

      lua_pushinteger(l, multiplier * value);
      1
    }
  }
}
