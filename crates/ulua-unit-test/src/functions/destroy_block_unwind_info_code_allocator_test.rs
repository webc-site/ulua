use core::ffi::c_void;

use crate::records::info_code_allocator_test::Info;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn destroy_block_unwind_info_code_allocator_test(
  context: *mut c_void,
  unwind_data: *mut c_void,
) {
  // Safety: CodeAllocator C ABI 回调契约：context 指向调用方存活 Info（非空、对齐、比帧长寿），unwind_data 是 create 回调交付的 Box::into_raw 堆指针——按契约所有权已移交本回调，Box::from_raw 唯一回收（无双重释放）；写 destroy_called 仅记账，帧内 &mut 无第二借用。
  unsafe {
    let info = &mut *(context.cast::<Info>());
    info.destroy_called = true;

    let value = Box::from_raw(unwind_data.cast::<i32>());
    assert_eq!(*value, 7);
  }
}
