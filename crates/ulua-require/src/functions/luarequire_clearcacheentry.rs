use ulua_vm::records::lua_state::lua_State;

use crate::functions::clear_cache_entry::clear_cache_entry;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe extern "C-unwind" fn luarequire_clearcacheentry(l: *mut lua_State) -> i32 {
  unsafe { clear_cache_entry(l) }
}
