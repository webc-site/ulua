use crate::{
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  type_aliases::{lua_destructor::LuaDestructor, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getuserdatadtor(l: *mut lua_State, tag: i32) -> LuaDestructor {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

  unsafe { (*(*l).global).udatagc[tag as usize] }
}
