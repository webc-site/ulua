use std::{ffi::c_void, sync::atomic::Ordering};

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::conformance_reference_dtor_hits::CONFORMANCE_REFERENCE_DTOR_HITS;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_reference_dtor(_l: *mut LuaState, _data: *mut c_void) {
  CONFORMANCE_REFERENCE_DTOR_HITS.fetch_add(1, Ordering::SeqCst);
}
