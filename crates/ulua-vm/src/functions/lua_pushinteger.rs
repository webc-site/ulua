use core::ffi::c_int;

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, cast_num::cast_num, setnvalue::setnvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub unsafe fn lua_pushinteger(l: *mut lua_State, n: c_int) {
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, cast_num!(n));
    api_incr_top!(l);
  }
}
