use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tovector::lua_tovector, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_checkvector(l: *mut lua_State, narg: c_int) -> *const f32 {
  unsafe {
    let v = lua_tovector(l, narg);
    if v.is_null() {
      tag_error(l, narg, LuaType::Vector as c_int);
    }
    v
  }
}
