use core::ffi::c_void;

use crate::records::allocation_data::AllocationData;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn allocation_callback_code_allocator_test(
  context: *mut c_void,
  old_pointer: *mut c_void,
  old_size: usize,
  new_pointer: *mut c_void,
  new_size: usize,
) {
  unsafe {
    let allocation_data = &mut *(context.cast::<AllocationData>());

    if !old_pointer.is_null() {
      assert_ne!(old_size, 0);
      allocation_data.bytes_freed += old_size;
    }

    if !new_pointer.is_null() {
      assert_ne!(new_size, 0);
      allocation_data.bytes_allocated += new_size;
    }
  }
}
