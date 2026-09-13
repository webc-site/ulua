use core::ffi::c_int;

use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned,
  },
  macros::{nbits::NBITS, trim::trim},
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_arshift(l: *mut lua_State) -> c_int {
  unsafe {
    let mut r: BUint = lua_l_checkunsigned(l, 1);
    let i: i32 = lua_l_checkinteger(l, 2);

    // C: `if (i < 0 || !(r & ((BUint)1 << (NBITS - 1))))` — logical NOT: sign bit clear.
    if i < 0 || (r & ((1 as BUint) << (NBITS as u32 - 1))) == 0 {
      // wrapping_neg avoids UB on INT_MIN (C++ negates a plain `int`).
      return b_shift(l, r, i.wrapping_neg());
    }

    // arithmetic shift for 'negative' number
    if i >= NBITS as c_int {
      r = !0 as BUint;
    } else {
      r = trim((r >> i as u32) | !(!(0 as BUint) >> i as u32)); // add signal bit
    }

    lua_pushunsigned(l, r);
    1
  }
}
