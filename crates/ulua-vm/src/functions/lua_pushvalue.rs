//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:310:lua_pushvalue`
//! Source: `VM/src/lapi.cpp:310-316` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{api_incr_top::api_incr_top, setobj_2_s::setobj2s},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushvalue(l: *mut lua_State, idx: c_int) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    let o: StkId = index2addr(l, idx);
    setobj2s!(l, (*l).top, o);
    api_incr_top!(l);
  }
}
