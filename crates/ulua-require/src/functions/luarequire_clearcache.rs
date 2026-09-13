use ulua_vm::records::lua_state::lua_State;

use crate::functions::clear_cache::clear_cache;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe extern "C-unwind" fn luarequire_clearcache(l: *mut lua_State) -> i32 {
  unsafe { clear_cache(l) }
}
