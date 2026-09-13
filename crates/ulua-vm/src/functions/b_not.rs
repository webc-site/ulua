use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  macros::trim::trim,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_not(l: *mut lua_State) -> c_int {
  unsafe {
    let r = !(lua_l_checkunsigned(l, 1));
    lua_pushunsigned(l, trim(r));
    1
  }
}
