use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_gc_set_block_allocations::conformance_gc_set_block_allocations,
  safe_api::{pushcclosurek, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_gc_setup(l: *mut LuaState) {
  pushcclosurek(
    l,
    Some(conformance_gc_set_block_allocations),
    Some(b"setblockallocations\0"),
    0,
    None,
  );
  state_mut(l).set_global_str("setblockallocations");
}
