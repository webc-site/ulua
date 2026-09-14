//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:206:lua_xpush`
//! Source: `VM/src/lapi.cpp:206-212` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::{index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj2s},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_xpush(from: *mut lua_State, to: *mut lua_State, idx: c_int) {
  unsafe {
    api_check!(from, (*from).global == (*to).global);
    lua_c_threadbarrier_lapi(to);
    let o = index2addr(from, idx);
    setobj2s!(to, (*to).top, o);
    api_incr_top!(to);
  }
}
