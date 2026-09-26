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
  // Safety: CodeAllocator 的 C ABI 回调契约：context 恒指向调用方持有的
  // AllocationData（非空、对齐、比回调帧长寿），old/new_pointer 为 null 或
  // 分配器产出的有效块且与对应 size 配对；&mut 重借用只在帧内做只增计数，
  // 单线程回调无第二借用。
  let allocation_data = unsafe { &mut *context.cast::<AllocationData>() };

  if !old_pointer.is_null() {
    assert_ne!(old_size, 0);
    allocation_data.bytes_freed += old_size;
  }

  if !new_pointer.is_null() {
    assert_ne!(new_size, 0);
    allocation_data.bytes_allocated += new_size;
  }
}
