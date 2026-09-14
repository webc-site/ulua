use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_hook(
  l: *mut lua_State,
  ar: *mut LuaDebug,
) {
  unsafe {
    assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), ar));
  }
}
