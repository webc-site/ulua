use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::trim::trim,
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_xor(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_gettop(l);
    let mut r: BUint = 0;

    for i in 1..=n {
      r ^= lua_l_checkunsigned(l, i);
    }

    lua_pushunsigned(l, trim(r));
    1
  }
}
