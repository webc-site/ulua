use core::ffi::c_int;

use crate::{
  functions::{lua_concat::lua_c_threadbarrier_lapi, lua_h_new::lua_h_new},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::luaC_checkGC,
    sethvalue::sethvalue,
  },
  records::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_createtable(l: *mut lua_State, narray: c_int, nrec: c_int) {
  unsafe {
    api_check!(l, narray >= 0 && nrec >= 0);
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    sethvalue!(l, (*l).top, lua_h_new(l, narray, nrec));
    api_incr_top!(l);
  }
}
