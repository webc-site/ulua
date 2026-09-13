use core::ffi::c_char;

use crate::{
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_s_newlstr::luaS_newlstr},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::luaC_checkGC,
    setsvalue::setsvalue,
  },
  records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushlstring(l: *mut lua_State, s: *const c_char, len: usize) {
  unsafe {
    api_check!(l, !s.is_null());
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    setsvalue!(l, (*l).top, luaS_newlstr(l, s, len));
    api_incr_top!(l);
  }
}
