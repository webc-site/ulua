use core::ffi::{c_int, c_void};

use ulua_vm::{
  functions::{lua_getcoverage::lua_getcoverage, lua_is_lfunction::lua_is_lfunction},
  macros::{lua_l_argexpected::luaL_argexpected, lua_newtable::lua_newtable},
  records::lua_state::lua_State,
};

use crate::common::functions::conformance_coverage_callback::conformance_coverage_callback;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_getcoverage(l: *mut lua_State) -> c_int {
  unsafe {
    luaL_argexpected!(l, lua_is_lfunction(l, 1) != 0, 1, "function");

    lua_newtable(l);
    lua_getcoverage(l, 1, l as *mut c_void, Some(conformance_coverage_callback));

    1
  }
}
