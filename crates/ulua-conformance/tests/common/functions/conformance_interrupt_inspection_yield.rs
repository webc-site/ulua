use core::{mem::zeroed, ptr::null_mut};

use ulua_vm::{
  functions::{lua_getinfo::lua_getinfo, luau_callhook::luau_callhook},
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};

use crate::common::functions::conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_yield(l: *mut lua_State) -> bool {
  unsafe {
    let mut ar: LuaDebug = zeroed();
    assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), &mut ar));

    luau_callhook(l, Some(conformance_interrupt_inspection_hook), null_mut());

    false
  }
}
