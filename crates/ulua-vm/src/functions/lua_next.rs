//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:1350:lua_next`
//! Source: `VM/src/lapi.cpp:1350-1363` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_h_next::lua_h_next,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    hvalue::hvalue, ttistable::ttistable,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_next(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);
    let t: StkId = index2addr(l, idx);
    api_check!(l, ttistable!(t));

    let more = lua_h_next(l, hvalue!(t), (*l).top.sub(1));
    if more != 0 {
      api_incr_top!(l);
    } else {
      (*l).top = (*l).top.sub(1);
    }
    more
  }
}
