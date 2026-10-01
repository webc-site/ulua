use core::ffi::{c_int, c_void};

use crate::common::functions::safe_api::udfield_setboolean;

use crate::common::records::vec_2_direct_field_access_test::Vec2;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn direct_field_access_get_non_zero_boolean(
  ud: *mut c_void,
  result: *mut c_void,
) {
  // Safety: `ud` 指向本用例的 Vec2 userdata 数据区（回调契约），仅本段读一次。
  let non_zero = unsafe { ((*(ud as *mut Vec2)).x != 0.0 || (*(ud as *mut Vec2)).y != 0.0) as c_int };
  udfield_setboolean(result, non_zero);
}
