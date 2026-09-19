use core::ffi::c_uint;

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, cast_num::cast_num, setnvalue::setnvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub(crate) unsafe fn lua_pushunsigned(l: *mut lua_State, u: c_uint) {
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, cast_num!(u));
    api_incr_top!(l);
  }
}
