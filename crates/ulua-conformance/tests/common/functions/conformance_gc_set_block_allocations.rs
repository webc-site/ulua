use core::ffi::c_int;

use ulua_vm::{functions::lua_l_checkboolean::lua_l_checkboolean, records::lua_state::lua_State};

use crate::common::functions::blockable_realloc_allowed::BLOCKABLE_REALLOC_ALLOWED;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_gc_set_block_allocations(l: *mut lua_State) -> c_int {
  unsafe {
    BLOCKABLE_REALLOC_ALLOWED = lua_l_checkboolean(l, 1) == 0;
  }

  0
}
