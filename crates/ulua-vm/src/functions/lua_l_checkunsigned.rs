use core::ffi::{c_int, c_uint};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tounsignedx::lua_tounsignedx, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_checkunsigned(l: *mut lua_State, narg: c_int) -> c_uint {
  unsafe {
    let mut isnum: i32 = 0;
    let d = lua_tounsignedx(l, narg, &mut isnum);
    if isnum == 0 {
      tag_error(l, narg, LuaType::Number as c_int);
    }
    d
  }
}
