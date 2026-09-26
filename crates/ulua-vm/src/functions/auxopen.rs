use core::{ffi::c_char, ptr::null};

use crate::{
  functions::lua_setfield::lua_setfield,
  macros::{lua_pushcclosure::lua_pushcclosure, lua_pushcfunction::LUA_PUSHCFUNCTION},
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

/// # Safety
/// `l` 必须是库 open 期间存活的调用帧，且栈顶（-2 处）已压入待填充的库表，供 `lua_setfield` 写入；
/// `name`/`f`/`u` 在本次调用内保持有效。违反将写坏栈布局或调到悬垂的 C 函数指针。cpp lbaselib.cpp:453。
pub(crate) unsafe fn auxopen(
  l: *mut LuaState,
  name: *const c_char,
  f: LuaCFunction,
  u: LuaCFunction,
) {
  // Safety: 契约保证 `L` 为库 open 期的存活调用帧，luaL_requiref 式注册仅在当前栈顶进行
  unsafe {
    LUA_PUSHCFUNCTION(l, u, null());
    lua_pushcclosure(l, f, name, 1);
    lua_setfield(l, -2, name);
  }
}
