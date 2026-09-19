//! Source: `VM/src/lapi.cpp:1942-1957`

use core::ffi::c_int;

use crate::{
  functions::{ensure_stack::ensure_stack, lua_concat::lua_c_threadbarrier_lapi},
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_utag_limit::LUA_UTAG_LIMIT,
    sethvalue::sethvalue, setnilvalue::setnilvalue,
  },
  records::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getuserdatametatable(l: *mut lua_State, tag: c_int) {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let h = (*(*l).global).udatamt[tag as usize];
    if !h.is_null() {
      sethvalue!(l, (*l).top, h);
    } else {
      setnilvalue!((*l).top);
    }

    api_incr_top!(l);
  }
}
