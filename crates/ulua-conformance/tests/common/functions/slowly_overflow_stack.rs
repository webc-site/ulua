use ulua_vm::{
  macros::luai_maxcstack::LUAI_MAXCSTACK, records::lua_state::LuaState,
};

use crate::common::functions::safe_api::{l_checkstack, state_mut};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn slowly_overflow_stack(l: *mut LuaState) -> i32 {
  for _ in 0..(LUAI_MAXCSTACK * 2) {
    l_checkstack(l, 1, "test");
    state_mut(l).push_number(1.0);
  }
  0
}
