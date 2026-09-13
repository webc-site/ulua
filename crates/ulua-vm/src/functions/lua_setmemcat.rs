use crate::{
  macros::{api_check::api_check, lua_memory_categories::LUA_MEMORY_CATEGORIES},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_setmemcat(l: *mut lua_State, category: i32) {
  api_check!(l, (category as u32) < LUA_MEMORY_CATEGORIES as u32);
  unsafe {
    (*l).activememcat = category as u8;
  }
}
