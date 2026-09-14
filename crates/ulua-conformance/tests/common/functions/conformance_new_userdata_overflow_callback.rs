use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_getmetatable::lua_getmetatable, lua_newuserdatadtor::lua_newuserdatadtor},
  records::lua_state::lua_State,
};

use crate::common::functions::conformance_new_userdata_overflow_dtor::conformance_new_userdata_overflow_dtor;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_new_userdata_overflow_callback(
  l: *mut lua_State,
) -> c_int {
  unsafe {
    lua_newuserdatadtor(l, usize::MAX, Some(conformance_new_userdata_overflow_dtor));
    lua_getmetatable(l, -1);

    0
  }
}
