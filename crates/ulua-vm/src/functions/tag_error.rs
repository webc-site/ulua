use crate::{
  functions::{lua_l_typeerror_l::lua_l_typeerror_l, lua_typename::lua_typename_str},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn tag_error(l: *mut lua_State, narg: i32, tag: i32) -> ! {
  let tname = lua_typename_str(tag).unwrap_or("unknown");
  unsafe { lua_l_typeerror_l(l, narg, tname) }
}
