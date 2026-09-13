use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek, macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
};

use crate::common::functions::{
  conformance_p_call_resume_error::conformance_p_call_resume_error, cxxthrow::cxxthrow,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_p_call_setup(l: *mut lua_State) {
  unsafe {
    lua_pushcclosurek(l, Some(cxxthrow), c"cxxthrow".as_ptr(), 0, None);
    lua_setglobal(l, c"cxxthrow".as_ptr());

    lua_pushcclosurek(
      l,
      Some(conformance_p_call_resume_error),
      c"resumeerror".as_ptr(),
      0,
      None,
    );
    lua_setglobal(l, c"resumeerror".as_ptr());
  }
}
