use core::ffi::{c_int, c_void};

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_lutag_limit::LUA_LUTAG_LIMIT,
    setpvalue::setpvalue,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub unsafe fn lua_pushlightuserdatatagged(l: *mut lua_State, p: *mut c_void, tag: c_int) {
  unsafe {
    // cpp `ensure_stack(L, 1)` 在 api_check 之前
    ensure_stack(l, 1);
    api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
    setpvalue!((*l).top, p, tag);
    api_incr_top!(l);
  }
}
