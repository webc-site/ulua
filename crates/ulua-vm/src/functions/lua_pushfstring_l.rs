use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_o_pushvfstring::luaO_pushvfstring},
  macros::lua_c_check_gc::luaC_checkGC,
  records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_pushfstring_l(
  l: *mut lua_State,
  fmt: *const c_char,
  argp: Arguments<'_>,
) -> *const c_char {
  unsafe {
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    luaO_pushvfstring(l, fmt, argp)
  }
}
