use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  macros::{nbits::NBITS, trim::trim},
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn b_rot(l: *mut lua_State, mut i: c_int) -> c_int {
  unsafe {
    let mut r: BUint = lua_l_checkunsigned(l, 1);

    // i = i % NBITS (avoid undefined shift when i == 0)
    i &= (NBITS - 1) as c_int;

    r = trim(r);
    if i != 0 {
      let i_u = i as u32;
      r = (r << i_u) | (r >> (NBITS as u32 - i_u));
    }

    lua_pushunsigned(l, trim(r));
    1
  }
}
