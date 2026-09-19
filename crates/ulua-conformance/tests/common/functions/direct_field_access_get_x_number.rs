use core::ffi::c_void;

use ulua_vm::functions::lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber;

use crate::common::records::vec_2_direct_field_access_test::Vec2;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_get_x_number(
  ud: *mut c_void,
  result: *mut c_void,
) {
  unsafe {
    lua_userdatadirectfield_setnumber(result, (*(ud as *mut Vec2)).x);
  }
}
