use core::ffi::c_int;

use crate::{
  functions::{b_rot::b_rot, lua_l_checkinteger::lua_l_checkinteger},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_lrot(l: *mut lua_State) -> c_int {
  unsafe { b_rot(l, lua_l_checkinteger(l, 2)) }
}
