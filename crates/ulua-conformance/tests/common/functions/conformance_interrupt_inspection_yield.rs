use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook,
  safe_api::{callhook, getinfo, zero_debug},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_yield(l: *mut LuaState) -> bool {
  let mut ar = zero_debug();
  assert_ne!(0, getinfo(l, 0, b"nsl", &mut ar));

  callhook(l, Some(conformance_interrupt_inspection_hook), None);

  false
}
