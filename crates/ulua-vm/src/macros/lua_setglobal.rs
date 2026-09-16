use core::ffi::c_char;

use crate::{
  functions::{lua_getfield::lua_getfield, lua_setfield::lua_setfield},
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn lua_setglobal(l: *mut lua_State, s: *const c_char) {
  unsafe {
    lua_setfield(l, LUA_GLOBALSINDEX, s);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn lua_getglobal(l: *mut lua_State, s: *const c_char) {
  unsafe {
    lua_getfield(l, LUA_GLOBALSINDEX, s);
  }
}
