use core::ffi::c_int;

use crate::{
  functions::lua_pushunsigned::lua_pushunsigned,
  macros::{nbits::NBITS, trim::trim},
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn b_shift(l: *mut lua_State, mut r: BUint, i: c_int) -> c_int {
  // Mirrors VM/src/lbitlib.cpp:b_shift
  if i < 0 {
    // Magnitude of the (right) shift. `i.unsigned_abs()` is defined for
    // `i == INT_MIN` (the C++ `i = -i` is UB there), and using it as the
    // bound also avoids a shift-by->=32 (itself UB) — |i| >= NBITS yields 0.
    let amount = i.unsigned_abs();
    r = trim(r);
    if amount >= NBITS as u32 {
      r = 0;
    } else {
      r >>= amount;
    }
  } else {
    if i >= NBITS as c_int {
      r = 0;
    } else {
      r <<= i as u32;
    }
    r = trim(r);
  }

  unsafe {
    lua_pushunsigned(l, r);
  }
  1
}
