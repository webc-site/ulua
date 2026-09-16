use core::ffi::{c_int, c_void};

use ulua_vm::functions::lua_userdatadirectfield_setboolean::lua_userdatadirectfield_setboolean;

use crate::common::records::vec_2_direct_field_access_test::Vec2;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_get_non_zero_boolean(
  ud: *mut c_void,
  result: *mut c_void,
) {
  unsafe {
    let vec = &*(ud as *mut Vec2);
    let non_zero = (vec.x != 0.0 || vec.y != 0.0) as c_int;
    lua_userdatadirectfield_setboolean(result, non_zero);
  }
}
