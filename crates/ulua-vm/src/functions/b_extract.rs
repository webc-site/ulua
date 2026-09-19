use core::ffi::c_int;

use crate::{
  functions::{
    fieldargs::fieldargs, lua_l_checkunsigned::lua_l_checkunsigned,
    lua_pushunsigned::lua_pushunsigned,
  },
  macros::mask::mask,
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_extract(l: *mut lua_State) -> c_int {
  unsafe {
    let r: BUint = lua_l_checkunsigned(l, 1);
    let (f, w) = fieldargs(l, 2);
    let r = (r >> f) & mask(w);
    lua_pushunsigned(l, r);
    1
  }
}
