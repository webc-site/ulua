use core::ffi::c_void;

use crate::{records::lua_state::lua_State, type_aliases::lua_alloc::LuaAlloc};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getallocf(l: *mut lua_State, ud: *mut *mut c_void) -> LuaAlloc {
  let f = unsafe { (*(*l).global).frealloc };
  if !ud.is_null() {
    unsafe { *ud = (*(*l).global).ud };
  }
  f
}
