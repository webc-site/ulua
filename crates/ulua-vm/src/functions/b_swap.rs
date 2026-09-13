use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_swap(l: *mut lua_State) -> c_int {
  unsafe {
    let n: BUint = lua_l_checkunsigned(l, 1);
    let n = (n << 24) | ((n << 8) & 0xff0000) | ((n >> 8) & 0xff00) | (n >> 24);
    lua_pushunsigned(l, n);
    1
  }
}
