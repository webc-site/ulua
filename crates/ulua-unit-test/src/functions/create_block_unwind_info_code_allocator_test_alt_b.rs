use core::{ffi::c_void, ptr};

use crate::records::info_code_allocator_test_alt_b::Info;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn create_block_unwind_info_code_allocator_test_alt_b(
  context: *mut c_void,
  block: *mut u8,
  _block_size: usize,
  begin_offset: &mut usize,
) -> *mut c_void {
  unsafe {
    let info = &mut *(context.cast::<Info>());

    assert_eq!(info.unwind.len(), 8);
    ptr::copy_nonoverlapping(info.unwind.as_ptr(), block, info.unwind.len());
    *begin_offset = 8;
    info.block = block;

    Box::into_raw(Box::new(7_i32)).cast()
  }
}
