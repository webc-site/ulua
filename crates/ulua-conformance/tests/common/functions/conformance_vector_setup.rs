use ulua_vm::records::lua_state::lua_State;

use crate::common::functions::{
  setup_native_helpers::setup_native_helpers, setup_vector_helpers::setup_vector_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_vector_setup(l: *mut lua_State) {
  unsafe {
    setup_vector_helpers(l);
    setup_native_helpers(l);
  }
}
