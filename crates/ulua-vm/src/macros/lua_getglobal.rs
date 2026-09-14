use core::ffi::{c_char, c_int};

use crate::{
  functions::lua_getfield::lua_getfield, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn lua_getglobal(l: *mut lua_State, s: *const c_char) -> c_int {
  unsafe { lua_getfield(l, LUA_GLOBALSINDEX, s) }
}
