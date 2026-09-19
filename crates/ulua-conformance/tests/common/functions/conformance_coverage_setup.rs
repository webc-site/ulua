use ulua_vm::{
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::lua_state::lua_State,
};

use crate::common::functions::conformance_coverage_getcoverage::conformance_coverage_getcoverage;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_setup(l: *mut lua_State) {
  unsafe {
    LUA_PUSHCFUNCTION(
      l,
      Some(conformance_coverage_getcoverage),
      c"getcoverage".as_ptr(),
    );
    lua_setglobal(l, c"getcoverage".as_ptr());
  }
}
