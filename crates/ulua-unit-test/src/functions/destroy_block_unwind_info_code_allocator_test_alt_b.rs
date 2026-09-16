use core::ffi::c_void;

use crate::records::info_code_allocator_test_alt_b::Info;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn destroy_block_unwind_info_code_allocator_test_alt_b(
  context: *mut c_void,
  unwind_data: *mut c_void,
) {
  unsafe {
    let info = &mut *(context.cast::<Info>());
    info.destroy_called = true;

    let value = Box::from_raw(unwind_data.cast::<i32>());
    assert_eq!(*value, 7);
  }
}
