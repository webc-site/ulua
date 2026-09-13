use core::ffi::c_int;

use crate::{
  functions::{lua_type::lua_type, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_checktype(l: *mut lua_State, narg: c_int, t: c_int) {
  unsafe {
    if lua_type(l, narg) != t {
      tag_error(l, narg, t);
    }
  }
}
