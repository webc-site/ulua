use ulua_vm::records::lua_state::lua_State;

use crate::functions::register_module_impl::register_module_impl;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe extern "C-unwind" fn luarequire_registermodule(l: *mut lua_State) -> i32 {
  unsafe { register_module_impl(l) }
}
