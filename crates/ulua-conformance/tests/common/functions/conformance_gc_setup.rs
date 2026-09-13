use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek, macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
};

use crate::common::functions::conformance_gc_set_block_allocations::conformance_gc_set_block_allocations;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_gc_setup(l: *mut lua_State) {
  unsafe {
    lua_pushcclosurek(
      l,
      Some(conformance_gc_set_block_allocations),
      c"setblockallocations".as_ptr(),
      0,
      None,
    );
    lua_setglobal(l, c"setblockallocations".as_ptr());
  }
}
