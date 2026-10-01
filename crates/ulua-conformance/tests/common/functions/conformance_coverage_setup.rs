use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_coverage_getcoverage::conformance_coverage_getcoverage,
  push_cfunction_global::push_cfunction_global,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_setup(l: *mut LuaState) {
  push_cfunction_global(l, Some(conformance_coverage_getcoverage), b"getcoverage\0");
}
