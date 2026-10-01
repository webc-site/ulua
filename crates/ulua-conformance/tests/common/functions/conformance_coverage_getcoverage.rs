use core::ffi::{c_int, c_void};

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_coverage_callback::conformance_coverage_callback,
  safe_api::{getcoverage, is_lfunction, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_getcoverage(l: *mut LuaState) -> c_int {
  state_mut(l).arg_expected(is_lfunction(l, 1) != 0, 1, "function");

  state_mut(l).new_table();
  getcoverage(l, 1, l as *mut c_void, Some(conformance_coverage_callback));

  1
}
