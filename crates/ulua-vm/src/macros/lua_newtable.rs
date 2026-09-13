use crate::{functions::lua_createtable::lua_createtable, records::lua_state::lua_State};

#[inline]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_newtable(l: *mut lua_State) {
  unsafe {
    lua_createtable(l, 0, 0);
  }
}
