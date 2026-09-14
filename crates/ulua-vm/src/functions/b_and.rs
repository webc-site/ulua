use core::ffi::c_int;

use crate::{
  functions::{andaux::andaux, lua_pushunsigned::lua_pushunsigned},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_and(l: *mut lua_State) -> c_int {
  unsafe {
    let r = andaux(l);
    lua_pushunsigned(l, r);
    1
  }
}
