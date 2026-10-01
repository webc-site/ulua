use core::ffi::c_void;

use crate::common::functions::safe_api::udfield_setnumber;

use crate::common::records::vec_2_direct_field_access_test::Vec2;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_get_x_number(
  ud: *mut c_void,
  result: *mut c_void,
) {
  // Safety: `ud` 指向本用例的 Vec2 userdata 数据区（回调契约），仅本行读一次。
  udfield_setnumber(result, unsafe { (*(ud as *mut Vec2)).x });
}
