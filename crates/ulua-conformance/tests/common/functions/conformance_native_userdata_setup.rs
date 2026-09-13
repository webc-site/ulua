use core::ptr::null_mut;

use ulua_code_gen::functions::set_userdata_remapper::set_userdata_remapper;
use ulua_vm::records::lua_state::lua_State;

use crate::common::functions::{
  conformance_native_userdata_remapper::conformance_native_userdata_remapper,
  setup_userdata_helpers::setup_userdata_helpers, setup_vector_helpers::setup_vector_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_native_userdata_setup(l: *mut lua_State) {
  unsafe {
    set_userdata_remapper(l, null_mut(), conformance_native_userdata_remapper);

    setup_vector_helpers(l);
    setup_userdata_helpers(l);
  }
}
