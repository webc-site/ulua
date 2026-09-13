use core::ffi::c_int;

use crate::{
  functions::{
    b_shift::b_shift, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_lshift(l: *mut lua_State) -> c_int {
  unsafe { b_shift(l, lua_l_checkunsigned(l, 1), lua_l_checkinteger(l, 2)) }
}
