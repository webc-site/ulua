use core::ffi::c_int;

use crate::{
  functions::{andaux::andaux, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_test(l: *mut lua_State) -> c_int {
  unsafe {
    let r = andaux(l);
    lua_pushboolean(l, if r != 0 { 1 } else { 0 });
    1
  }
}
