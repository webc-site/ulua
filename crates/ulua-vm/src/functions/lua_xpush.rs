//! Source: `VM/src/lapi.cpp:206-212` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::{
    ensure_stack::ensure_stack_impl, index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj2s},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_xpush(from: *mut lua_State, to: *mut lua_State, idx: c_int) {
  unsafe {
    api_check!(from, (*from).global == (*to).global);
    lua_c_threadbarrier_lapi(to);
    // cpp `ensure_stack_impl(to, from, 1)`
    ensure_stack_impl(to, from, 1);
    let o = index2addr(from, idx);
    setobj2s!(to, (*to).top, o);
    api_incr_top!(to);
  }
}
