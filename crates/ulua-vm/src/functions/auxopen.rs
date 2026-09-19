use core::{ffi::c_char, ptr::null};

use crate::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_pushcclosure::lua_pushcclosure, lua_pushcfunction::LUA_PUSHCFUNCTION},
  type_aliases::{lua_c_function::LuaCFunction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn auxopen(
  l: *mut lua_State,
  name: *const c_char,
  f: LuaCFunction,
  u: LuaCFunction,
) {
  unsafe {
    LUA_PUSHCFUNCTION(l, u, null());
    lua_pushcclosure(l, f, name, 1);
    lua_setfield(l, -2, name);
  }
}
